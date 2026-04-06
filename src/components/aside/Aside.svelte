<script lang="ts">
  import type { PortInfo } from "../../interfaces/Ports";

  interface Props {
    selectedPort: PortInfo | null;
    watchedPorts: Set<number>;
    blockedPids: Set<number>;
    onToggleWatch: (port: PortInfo) => Promise<void>;
    onToggleInternet: (port: PortInfo) => Promise<void>;
    onKillProcess: (port: PortInfo) => Promise<void>;
  }

  let {
    selectedPort,
    watchedPorts,
    blockedPids,
    onToggleWatch,
    onToggleInternet,
    onKillProcess,
  }: Props = $props();

  let isWatched = $derived(
    selectedPort ? watchedPorts.has(selectedPort.local_port) : false,
  );

  let isBlocked = $derived(
    selectedPort?.pid ? blockedPids.has(selectedPort.pid) : false,
  );
</script>

<aside class="basis-[40%] shrink-0 border border-red-500 p-4 overflow-auto">
  {#if selectedPort}
    <div class="text-white">
      <!-- Header -->
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-xl font-bold text-emerald-400">Port Details</h2>
      </div>

      <!-- Port Number - Large Display -->
      <div class="mb-6 text-center">
        <div class="text-5xl font-bold text-emerald-400 mb-2">
          {selectedPort.local_port}
        </div>
        <div class="text-sm text-gray-400">Port Number</div>
      </div>

      <!-- Details Grid -->
      <div class="space-y-4">
        <!-- Protocol & State -->
        <div class="grid grid-cols-2 gap-4">
          <div class="bg-gray-800 p-3 rounded">
            <div class="text-xs text-gray-400 mb-1">Protocol</div>
            <div class="font-semibold text-lg">
              <span
                class="px-3 py-1 rounded {selectedPort.protocol === 'TCP'
                  ? 'bg-blue-900 text-blue-200'
                  : 'bg-purple-900 text-purple-200'}"
              >
                {selectedPort.protocol}
              </span>
            </div>
          </div>

          <div class="bg-gray-800 p-3 rounded">
            <div class="text-xs text-gray-400 mb-1">State</div>
            <div class="font-semibold text-lg">
              <span
                class="px-3 py-1 rounded text-sm {selectedPort.state ===
                'Listen'
                  ? 'bg-green-900 text-green-200'
                  : selectedPort.state === 'Established'
                    ? 'bg-blue-900 text-blue-200'
                    : selectedPort.state === 'TimeWait'
                      ? 'bg-yellow-900 text-yellow-200'
                      : 'bg-gray-700 text-gray-300'}"
              >
                {selectedPort.state}
              </span>
            </div>
          </div>
        </div>

        <!-- Local Address -->
        <div class="bg-gray-800 p-3 rounded">
          <div class="text-xs text-gray-400 mb-1">Local Address</div>
          <div class="font-mono text-sm break-all">
            {selectedPort.local_address}
          </div>
        </div>

        <!-- Remote Address -->
        <div class="bg-gray-800 p-3 rounded">
          <div class="text-xs text-gray-400 mb-1">Remote Address</div>
          <div class="font-mono text-sm break-all">
            {selectedPort.remote_address}
          </div>
        </div>

        <!-- Remote Port -->
        <div class="bg-gray-800 p-3 rounded">
          <div class="text-xs text-gray-400 mb-1">Remote Port</div>
          <div class="font-mono text-sm break-all">
            {selectedPort.remote_port}
          </div>
        </div>

        <!-- Remote Connection -->
        {#if selectedPort.remote_port > 0}
          <div class="bg-gray-800 p-3 rounded">
            <div class="text-xs text-gray-400 mb-1">Remote Address</div>
            <div class="font-mono text-sm break-all">
              {selectedPort.remote_address}
            </div>
          </div>

          <div class="bg-gray-800 p-3 rounded">
            <div class="text-xs text-gray-400 mb-1">Remote Port</div>
            <div class="font-mono text-lg font-semibold">
              {selectedPort.remote_port}
            </div>
          </div>
        {/if}

        <!-- Process Information -->
        <div class="bg-gray-800 p-3 rounded">
          <div class="text-xs text-gray-400 mb-1">Process</div>
          <div class="font-semibold break-all">
            {selectedPort.process_name || "Unknown Process"}
          </div>
        </div>

        {#if selectedPort.pid}
          <div class="bg-gray-800 p-3 rounded">
            <div class="text-xs text-gray-400 mb-1">Process ID (PID)</div>
            <div class="font-mono text-lg font-semibold">
              {selectedPort.pid}
            </div>
          </div>
        {/if}

        {#if selectedPort.command_line}
          <div class="bg-gray-800 p-3 rounded">
            <div class="text-xs text-gray-400 mb-1">Command Line</div>
            <div
              class="font-mono text-xs break-all text-gray-200 whitespace-pre-wrap"
            >
              {selectedPort.command_line}
            </div>
          </div>
        {/if}

        <!-- Connection Type Info -->
        <div class="bg-gray-800 p-3 rounded">
          <div class="text-xs text-gray-400 mb-1">Connection Type</div>
          <div class="text-sm">
            {#if selectedPort.local_address.startsWith("127.0.0.1") || selectedPort.local_address.startsWith("::1")}
              <span class="text-blue-300">🏠 Localhost</span>
              <div class="text-xs text-gray-400 mt-1">
                Only accessible from this machine
              </div>
            {:else if selectedPort.local_address === "0.0.0.0" || selectedPort.local_address === "::"}
              <span class="text-yellow-300">🌍 All Interfaces</span>
              <div class="text-xs text-gray-400 mt-1">
                Listening on all network interfaces
              </div>
            {:else}
              <span class="text-green-300">🔌 Network</span>
              <div class="text-xs text-gray-400 mt-1">
                Accessible from network
              </div>
            {/if}
          </div>
        </div>

        <!-- Port Range Category -->
        <div class="bg-gray-800 p-3 rounded">
          <div class="text-xs text-gray-400 mb-1">Port Category</div>
          <div class="text-sm">
            {#if selectedPort.local_port < 1024}
              <span class="text-red-300">⚠️ Well-Known Port (1-1023)</span>
              <div class="text-xs text-gray-400 mt-1">
                System/privileged services
              </div>
            {:else if selectedPort.local_port < 49152}
              <span class="text-blue-300">📋 Registered Port (1024-49151)</span>
              <div class="text-xs text-gray-400 mt-1">
                User/application services
              </div>
            {:else}
              <span class="text-gray-300">🔄 Ephemeral Port (49152-65535)</span>
              <div class="text-xs text-gray-400 mt-1">
                Temporary/dynamic connections
              </div>
            {/if}
          </div>
        </div>
      </div>

      <button
        class="w-full px-4 py-2 rounded font-semibold flex items-center
   justify-center gap-2 {isWatched
          ? 'bg-emerald-700 hover:bg-emerald-800 text-white'
          : 'bg-gray-700 hover:bg-gray-600 text-white'}
      {!selectedPort?.command_line ? 'opacity-40 cursor-not-allowed' : ''}"
        onclick={() => selectedPort && onToggleWatch(selectedPort)}
        disabled={!selectedPort?.command_line}
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="1em"
          height="1em"
          viewBox="0 0 24 24"
        >
          <g
            fill="none"
            stroke="currentColor"
            stroke-linecap="round"
            stroke-width="1.5"
          >
            <path
              stroke-miterlimit="10"
              d="M18.024 7.043A8.374 8.374 0
   0 0 3.74 12.955"
            />
            <path
              stroke-linejoin="round"
              d="m17.35 2.75l.832
  3.372a1.123 1.123 0 0 1-.854 1.382l-3.372.843"
            />
            <path
              stroke-miterlimit="10"
              d="M5.976 16.957a8.374 8.374 0
   0 0 14.285-5.912"
            />
            <path
              stroke-linejoin="round"
              d="m6.65
  21.25l-.832-3.372a1.124 1.124 0 0 1 .855-1.382l3.371-.843"
            />
          </g>
        </svg>
        {isWatched ? "Stop Watching" : "Keep Alive"}
      </button>

      <!-- Internet Access Toggle -->
      <div
        class="bg-gray-800 p-3 rounded flex items-center justify-between mb-4"
      >
        <div>
          <div class="text-xs text-gray-400 mb-1">Internet Access</div>
          <div
            class="text-sm font-semibold {isBlocked
              ? 'text-red-400'
              : 'text-green-400'}"
          >
            {isBlocked ? "Blocked" : "Allowed"}
          </div>
        </div>
        <button
          aria-label="Toggle internet access"
          role="switch"
          aria-checked={isBlocked}
          onclick={() => selectedPort && onToggleInternet(selectedPort)}
          disabled={!selectedPort?.pid}
          class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors duration-200
                 {isBlocked ? 'bg-red-600' : 'bg-gray-600'}
                 {!selectedPort?.pid
            ? 'opacity-40 cursor-not-allowed'
            : 'cursor-pointer'}"
        >
          <span
            class="inline-block h-4 w-4 rounded-full bg-white transition-transform duration-200
                       {isBlocked ? 'translate-x-6' : 'translate-x-1'}"
          >
          </span>
        </button>
      </div>

      <!-- Action Buttons -->
      <div class="mt-6 space-y-2">
        <button
          class="w-full px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded font-semibold"
          onclick={() => selectedPort && onKillProcess(selectedPort)}
        >
          🔪 Kill Process
        </button>
        <button
          class="w-full px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded"
          onclick={() => {
            const info = `Port: ${selectedPort?.local_port}\nProtocol:
  ${selectedPort?.protocol}\nState: ${selectedPort?.state}\nProcess:
  ${selectedPort?.process_name}\nLocal: ${selectedPort?.local_address}\nRemote:
  ${selectedPort?.remote_address}:${selectedPort?.remote_port}`;
            navigator.clipboard.writeText(info);
            alert("Port info copied to clipboard!");
          }}
        >
          📋 Copy Info
        </button>
      </div>
    </div>
  {:else}
    <div class="text-gray-400 text-center mt-20">
      <div class="text-6xl mb-4">🔍</div>
      <h2 class="text-xl font-semibold mb-2">No Port Selected</h2>
      <p class="text-sm">Click on a port in the table to view details</p>
    </div>
  {/if}
</aside>
