<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main
  data-tauri-drag-region
  class="bg-[rgba(31,31,31,0.95)] w-full h-screen flex flex-col p-2"
>
  <header
    data-tauri-drag-region
    class="border-2 border-amber-500 text-white bg-amber-400/20"
  >
    <h3 class="bg-amber-900 inline">Header</h3>
  </header>

  <div class="flex border-2 border-blue-500 w-full h-full">
    <section class="border-2 border-green-500 w-full">
      <h1 class="text-3xl font-bold mb-4 text-emerald-600">
        Welcome to Tauri + Svelte
      </h1>
    </section>
    <aside class="border border-red-500 w-full">
      <h2 class="text-white">aside</h2>
    </aside>
  </div>
</main>
