<script lang="ts">
  import Aside from "../components/aside/Aside.svelte";
  import ShowPorts from "../components/content/ShowPorts.svelte";

  interface PortInfo {
    local_port: number;
    local_address: string;
    remote_port: number;
    remote_address: string;
    state: string;
    protocol: string;
    process_name: string | null;
    pid: number | null;
  }

  let selectedPort = $state<PortInfo | null>(null);

  function handlePOrtSleceted(port: PortInfo | null) {
    selectedPort = port;
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
    <ShowPorts onPortSelected={handlePOrtSleceted} />

    <Aside selectedPort={selectedPort!} />
  </div>
</main>
