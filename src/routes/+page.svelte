<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let basic_y_pos = $state(0);
  let smart_y_pos = $state(0);
</script>

<header>
  <h1>Smart vs normal elevator</h1>
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
  <img src="/stick-figure.svg" alt="person" id="person">
  <img src="/elevators/smart-elevator.svg" alt="smart elevator" id="smart-elevator">
  <img src="/elevators/normal-elevator.svg" alt="normal elevator" id="normal-elevator">
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

  #person {
    position: absolute;
    top: 85.5vh;
    left: 8%;
    transform: translate(-50%);
    height: 10vh;
  }
</style>
