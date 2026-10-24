<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let basic_y_pos = $state(0);
  let smart_y_pos = $state(0);
</script>

<header>
  <h1>The difference between a smart elevator and a normal elevator</h1>
</header>

<main>
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
  font-size: 16px;
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

#smart-elevator {
  position: absolute;
  top: 70%;
  left: 25%;
  transform: translate(-50%);
  width: 12vw;
  -webkit-filter: drop-shadow(0px 5px 5px var(--shadow-color));
  filter: drop-shadow(0px 5px 5px var(--shadow-color));
}

#normal-elevator {
  position: absolute;
  top: 70%;
  left: 75%;
  transform: translate(-50%);
  width: 12vw;
  -webkit-filter: drop-shadow(0px 5px 5px var(--shadow-color));
  filter: drop-shadow(0px 5px 5px var(--shadow-color));
}

@media (max-width: 768px) {
  #smart-elevator {
    width: 40vw;
  }

  #normal-elevator {
    width: 40vw;
  }
}
</style>
