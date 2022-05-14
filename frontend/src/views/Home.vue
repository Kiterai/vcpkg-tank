<template>
  <div class="center space-tb">
    <h1>Vcpkg Tank</h1>
    <p>
      <input v-model="input_pkglist" autofocus class="input-pkglist" type="text" />
    </p>
    <p>
      <button class="btn-download" @click="DownloadRequest">Download</button>
    </p>
    <ul>
      <li v-for="task in tasks" :key="task.id" class="task">{{ task.pkgs }}</li>
    </ul>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";

export default defineComponent({
  name: "Home",
  methods: {
    DownloadRequest() {
      if (!this.input_pkglist) {
        return;
      }

      fetch("/api/export", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          pkgs: [this.input_pkglist],
        }),
      })
        .then((res) => {
          return res.json();
        })
        .then((res) => {
          const id = res.id;

          this.tasks.push({
            id: id,
            pkgs: this.input_pkglist,
          });
        });
    },
  },
  data(): {
    tasks: Array<{
      id: string;
      pkgs: string;
    }>;
    input_pkglist: string;
  } {
    return {
      tasks: [],
      input_pkglist: "",
    };
  },
});
</script>

<style lang="scss" scoped>
.center {
  text-align: center;
}

.space-tb {
  margin: 5rem 0;
}

.input-pkglist {
  font-size: 2rem;
  font-weight: bold;
  padding: 0.8rem;
  border-radius: 0.8rem;
  width: 20em;
  max-width: 90%;
}

.btn-download {
  border: 0.4rem solid #000;
  border-radius: 1.2rem;
  font-size: 1.5rem;
  font-weight: bold;
  padding: 0.8rem;
  background: #000;
  color: #fff;
  cursor: pointer;
  transition: background-color 0.2s ease;

  &:hover {
    background: #fff;
    color: #000;
  }
}

ul {
  padding: 0;
}

.task {
  display: block;
  font-weight: bold;
  font-size: 1.2rem;
}
</style>
