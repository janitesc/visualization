<script lang="ts">
  import { getGenes, Genes } from "../api.ts";

  export let width: number;
  export let height: number;

  console.debug(getGenes());
</script>

<style>
  canvas {
    width: 100%;
    height: 100%;
    border: 2px solid black;
  }

  div {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
  }
</style>

<div>
  <h1>Genes</h1>
  <canvas width={width} height={height}>
  </canvas>
</div>
