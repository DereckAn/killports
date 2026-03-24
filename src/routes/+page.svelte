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

<main class="bg-[rgba(18,18,20,0.71)]">
  <h1 class="text-3xl font-bold mb-4 text-emerald-600 ">Welcome to Tauri + Svelte</h1>
</main>
