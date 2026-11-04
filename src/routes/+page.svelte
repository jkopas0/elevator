<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";

  type StepInfo = {
    position: number;
    floors: number[];
    inside: number;
    distance_travelled: number;
  };

  let basic_y_pos = $state(0);
  let smart_y_pos = $state(0);
  let basic_floors = $state<number[]>([]);
  let smart_floors = $state<number[]>([]);
  let basic_inside = $state(0);
  let smart_inside = $state(0);
  let basic_distance = $state(0);
  let smart_distance = $state(0);

  let status = $state<"idle" | "running" | "done">("idle");
  let error = $state<string | null>(null);
  let dragging = $state(false);

  $effect(() => {
    const unlisten = listen<[StepInfo, StepInfo]>("step", (event) => {
      const [basic, smart] = event.payload;
      basic_y_pos = basic.position;
      smart_y_pos = smart.position;
      basic_floors = basic.floors;
      smart_floors = smart.floors;
      basic_inside = basic.inside;
      smart_inside = smart.inside;
      basic_distance = basic.distance_travelled;
      smart_distance = smart.distance_travelled;
    });
    return () => { unlisten.then((f) => f()); };
  });

  $effect(() => {
    const unlisten = listen("done", () => { status = "done"; });
    return () => { unlisten.then((f) => f()); };
  });

  // Počítadlo pro DragEnter/Leave (aby DragLeave na child nedeaktivoval overlay)
  let dragCounter = 0;

  $effect(() => {
    function onDragEnter(e: DragEvent) {
      e.preventDefault();
      dragCounter++;
      if (status === "idle" || status === "done") dragging = true;
    }
    function onDragOver(e: DragEvent) {
      e.preventDefault();
    }
    function onDragLeave(e: DragEvent) {
      e.preventDefault();
      dragCounter--;
      if (dragCounter <= 0) {
        dragCounter = 0;
        dragging = false;
      }
    }
    function onDrop(e: DragEvent) {
      e.preventDefault();
      dragCounter = 0;
      dragging = false;

      if (status === "running") return;

      const file = e.dataTransfer?.files[0];
      if (!file) return;
      if (!file.name.endsWith(".json")) {
        error = "Soubor musí být ve formátu .json";
        return;
      }
      const reader = new FileReader();
      reader.onload = () => loadJson(reader.result as string);
      reader.readAsText(file);
    }

    window.addEventListener("dragenter", onDragEnter);
    window.addEventListener("dragover", onDragOver);
    window.addEventListener("dragleave", onDragLeave);
    window.addEventListener("drop", onDrop);
    return () => {
      window.removeEventListener("dragenter", onDragEnter);
      window.removeEventListener("dragover", onDragOver);
      window.removeEventListener("dragleave", onDragLeave);
      window.removeEventListener("drop", onDrop);
    };
  });

  async function loadJson(json: string) {
    error = null;
    try {
      await invoke("load", { json });
      status = "running";
    } catch (e) {
      error = String(e);
      status = "idle";
    }
  }

  function handleFileInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = () => loadJson(reader.result as string);
    reader.readAsText(file);
    input.value = "";
  }
</script>

<header>
  <h1>Smart vs normal elevator</h1>
  {#if status !== "idle"}
    <div class="stats">
      <span>Smart: {smart_distance} floors travelled</span>
      <span>Normal: {basic_distance} floors travelled</span>
    </div>
  {/if}
</header>

{#if status === "idle" || status === "done"}
  <!-- Překryv přes celou obrazovku pro drag & drop -->
  <div class="screen-overlay" class:dragging>
    <div class="dropzone" class:done={status === "done"}>
      {#if status === "done"}
        <p class="done-label">Simulace dokončena!</p>
        <p>Přetáhněte nový JSON soubor</p>
        <p class="or">nebo</p>
      {:else}
        <p>Přetáhněte JSON soubor sem</p>
        <p class="or">nebo</p>
      {/if}
      <label class="file-btn">
        Vybrat soubor
        <input type="file" accept=".json" onchange={handleFileInput} hidden />
      </label>
      {#if error}
        <p class="error">{error}</p>
      {/if}
    </div>
  </div>
{/if}

{#if status === "running" || status === "done"}
  <main>
    <div class="floors floors-left" id="floor1"></div>
    <div class="floors floors-left" id="floor2"></div>
    <div class="floors floors-left" id="floor3"></div>
    <div class="floors floors-left" id="floor4"></div>
    <div class="floors floors-left" id="floor5"></div>
    <div class="floors floors-left" id="floor6"></div>
    <div class="floors floors-left" id="floor7"></div>
    <div class="floors floors-left" id="floor8"></div>
    <div class="floors floors-right" id="floor1"></div>
    <div class="floors floors-right" id="floor2"></div>
    <div class="floors floors-right" id="floor3"></div>
    <div class="floors floors-right" id="floor4"></div>
    <div class="floors floors-right" id="floor5"></div>
    <div class="floors floors-right" id="floor6"></div>
    <div class="floors floors-right" id="floor7"></div>
    <div class="floors floors-right" id="floor8"></div>
    {#each smart_floors as count, floor}
      {#if count > 0}
        <div class="waiting waiting-left" style:top="{85.5 - 11 * floor}vh">
          <img src="/stick-figure.svg" alt="person">
          <span class="count">{count}</span>
        </div>
      {/if}
    {/each}
    {#each basic_floors as count, floor}
      {#if count > 0}
        <div class="waiting waiting-right" style:top="{85.5 - 11 * floor}vh">
          <img src="/stick-figure.svg" alt="person">
          <span class="count">{count}</span>
        </div>
      {/if}
    {/each}
    <img src="/elevators/smart-elevator.svg" alt="smart elevator" id="smart-elevator" style:top="{83.5 - 11 * smart_y_pos}vh">
    <span class="count inside inside-left" style:top="{87 - 11 * smart_y_pos}vh">{smart_inside}</span>
    <img src="/elevators/normal-elevator.svg" alt="normal elevator" id="normal-elevator" style:top="{83.5 - 11 * basic_y_pos}vh">
    <span class="count inside inside-right" style:top="{87 - 11 * basic_y_pos}vh">{basic_inside}</span>
  </main>
{/if}

<style>
  @font-face {
    font-family: 'Bricolage Grotesque';
    src: url('/fonts/BricolageGrotesque.woff2') format('woff2');
    font-weight: 200 800;
    font-stretch: 75% 100%;
    font-style: normal;
    font-display: swap;
  }

  :root {
    font-family: 'Bricolage Grotesque', system-ui, -apple-system, 'Segoe UI', sans-serif;
    font-size: 11px;
    line-height: 24px;
    font-weight: 400;

    background-color: #2f2f2f;
    color: #f6f6f6;

    --shadow-color: #1f1f1f;
    --header-border: #4f4f4f;
    --header-background: #434343;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  * {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  header {
    padding: 1em;
    text-align: center;
    background-color: var(--header-background);
    border: 1px solid var(--header-border);
    border-radius: 10px;
    box-shadow: 0px 5px 5px var(--shadow-color);
  }

  main {
    display: flex;
    align-items: center;
    flex-direction: column;
  }

  .screen-overlay {
    position: fixed;
    inset: 0;
    z-index: 10;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: rgba(0, 0, 0, 0.5);
    transition: background-color 0.15s;
  }

  .screen-overlay.dragging {
    background-color: rgba(0, 30, 60, 0.75);
  }

  .dropzone {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.8em;
    padding: 2.5em 3em;
    background-color: var(--header-background);
    border: 2px dashed var(--header-border);
    border-radius: 16px;
    box-shadow: 0px 8px 24px var(--shadow-color);
    text-align: center;
    min-width: 320px;
    pointer-events: none;
    transition: border-color 0.15s;
  }

  .dropzone .file-btn {
    pointer-events: all;
  }

  .screen-overlay.dragging .dropzone {
    border-color: #7eb8ff;
    border-style: solid;
  }

  .dropzone.done {
    border-color: #6fcf6f;
    border-style: solid;
  }

  .dropzone p {
    font-size: 1.5em;
    color: #ccc;
  }

  .dropzone .or {
    font-size: 1.2em;
    color: #888;
  }

  .done-label {
    font-size: 2em !important;
    color: #6fcf6f !important;
    font-weight: 700;
  }

  .file-btn {
    cursor: pointer;
    padding: 0.6em 1.4em;
    background-color: #5a7fa8;
    color: #fff;
    border-radius: 8px;
    font-size: 1.4em;
    transition: background-color 0.15s;
  }

  .file-btn:hover {
    background-color: #7eb8ff;
  }

  .error {
    color: #ff6b6b !important;
    font-size: 1.2em !important;
  }

  /* --- Floors & Elevators --- */
  .floors {
    position: absolute;
    transform: translate(-50%);
    background-color: var(--header-background);
    width: 45vw;
    height: 0.5vh;
  }

  .floors-left {
    left: 25vw;
  }

  .floors-right {
    left: 75vw;
  }

  #floor1 { top: 95vh; }
  #floor2 { top: 84vh; }
  #floor3 { top: 73vh; }
  #floor4 { top: 62vh; }
  #floor5 { top: 51vh; }
  #floor6 { top: 40vh; }
  #floor7 { top: 29vh; }
  #floor8 { top: 18vh; }

  #smart-elevator {
    position: absolute;
    top: 83.5vh;
    left: 25%;
    transform: translate(-50%);
    height: 12.5vh;
  }

  #normal-elevator {
    position: absolute;
    top: 83.5vh;
    left: 75%;
    transform: translate(-50%);
    height: 12.5vh;
  }

  .waiting {
    position: absolute;
    display: flex;
    align-items: center;
    transform: translate(-50%);
    height: 10vh;
  }

  .waiting img {
    height: 100%;
  }

  .waiting-left {
    left: 8%;
  }

  .waiting-right {
    left: 58%;
  }

  .count {
    font-size: 2.5em;
    font-weight: 700;
  }

  .stats {
    display: flex;
    justify-content: space-around;
  }

  .inside {
    position: absolute;
  }

  .inside-left {
    left: 31%;
  }

  .inside-right {
    left: 81%;
  }
</style>
