<template>
  <div class="center space-tb">
    <h1>Vcpkg Tank</h1>
    <p>
      <input v-model="input_pkglist" autofocus class="input-pkglist wide" type="text" />
    </p>
    <p>
      <button class="btn btn-request" @click="DownloadRequest" :disabled="!input_pkglist">Download</button>
    </p>
    <ul>
      <li v-for="task in tasks" :key="task.id" class="task wide">
        <div class="task-name">{{ task.pkgs }}</div>
        <div class="task-info">
          <div v-if="task.error" class="btn btn-error">Error</div>
          <button v-else-if="task.loading" class="btn btn-download" disabled>Loading...</button>
          <a v-else class="btn btn-download" :href="task.url">Download</a>
        </div>
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "Home",
  methods: {
    DownloadRequest() {
      if (!this.input_pkglist) {
        return;
      }
      const target_pkglist = this.input_pkglist;
      this.input_pkglist = "";

      fetch("/api/export", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          pkgs: [target_pkglist],
        }),
      })
        .then((res) => {
          return res.json();
        })
        .then((res) => {
          const id = res.id;

          this.tasks.push({
            id: id,
            pkgs: target_pkglist,
            loading: true,
            error: false,
            url: "",
          });

          const timer = setInterval(() => {
            const i = this.tasks.findIndex((task) => {
              return task.id == id;
            });

            const controller = new AbortController();
            const abort_timer = setTimeout(() => {
              controller.abort();
            }, 800);

            fetch(`/api/export?id=${id}`, {
              method: "HEAD",
              signal: controller.signal,
            })
              .then((res) => {
                if (res.status == 200) {
                  this.tasks[i].loading = false;
                  this.tasks[i].url = `/api/export?id=${id}`;
                  clearInterval(timer);
                } else if (res.status >= 400) {
                  this.tasks[i].error = true;
                  clearInterval(timer);
                }
              });
          }, 2000);
        });
    },
  },
  data(): {
    tasks: Array<{
      id: string;
      pkgs: string;
      loading: boolean;
      error: boolean;
      url: string;
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

.wide {
  width: 40rem;
  max-width: 90%;
  margin-left: auto;
  margin-right: auto;
}

.input-pkglist {
  font-size: 2rem;
  font-weight: bold;
  padding: 0.8rem;
  border-radius: 0.8rem;
}

.btn {
  border: solid #000;
  font-weight: bold;
  background: #000;
  color: #fff;
  cursor: pointer;
  transition: background-color 0.2s ease, border-color 0.2s ease;
  text-decoration: none;
  font-family: sans-serif;
  display: inline-block;

  &:hover {
    background: #fff;
    color: #000;
  }
}

.btn[disabled] {
  cursor: default;
  background: #888;
  border-color: #888;

  &:hover {
    color: #fff;
  }
}

.btn-request {
  font-size: 1.5rem;
  padding: 0.8rem;
  border-radius: 1.2rem;
  border-width: 0.4rem;
}

.btn-download {
  font-size: 1.2rem;
  padding: 0 0.2rem;
  border-radius: 0.5rem;
  border-width: 0.2rem;
}

.btn-error {
  font-size: 1.2rem;
  padding: 0 0.2rem;
  border-radius: 0.5rem;
  border-width: 0.2rem;
  border-color: #fa7;
  background-color: #fa7;
}

ul {
  padding: 0;
}

.task {
  display: flex;
  font-weight: bold;
  font-size: 1.2rem;
  margin-top: 0.5rem;
}

.task-name {
  width: calc(100% - 8rem);
  word-wrap: break-word;
}
.task-info {
  width: 8rem;
  align-self: center;
}
</style>
