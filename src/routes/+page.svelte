<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let basic_y_pos = $state(0);
  let smart_y_pos = $state(0);
</script>

<main>
<h1>The difference between a smart elevator and a normal elevator</h1>
<div id="elevator-smart"></div>
<div id="elevator-normal"></div>
</main>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  background-color: #2f2f2f;
  color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

#elevator-smart {
  position: absolute;
  top: 75%;
  left: 25%;
  transform: translate(-50%);
  width: 10em;
  height: 10em;
  background-color: #AAA;
}

#elevator-normal {
  position: absolute;
  top: 75%;
  left: 75%;
  transform: translate(-50%);
  width: 10em;
  height: 10em;
  background-color: #AAA;
}
</style>
