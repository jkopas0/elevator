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
    return () => {
      unlisten.then((f) => f());
    };
  });

  invoke("load", { path: "../data/test.json" }).catch(console.error);
</script>

<header>
  <h1>Smart vs normal elevator</h1>
  <div class="stats">
    <span>Smart: {smart_distance} floors travelled</span>
    <span>Normal: {basic_distance} floors travelled</span>
  </div>
</header>

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

  #floor1 {
    top: 95vh;
  }

  #floor2 {
    top: 84vh;
  }

  #floor3 {
    top: 73vh;
  }

  #floor4 {
    top: 62vh;
  }

  #floor5 {
    top: 51vh;
  }

  #floor6 {
    top: 40vh;
  }

  #floor7 {
    top: 29vh;
  }

  #floor8 {
    top: 18vh;
  }

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
