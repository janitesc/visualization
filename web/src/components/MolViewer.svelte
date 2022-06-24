<script lang="ts">
    import { getmol, mol } from "../api.ts";
    import App from "../App.svelte";
    export let coordData = [];
    export let bondData = [];
    console.debug(getmol());
    let mult = 100;
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
    svg {
      align-self: center;
          overflow: visible;
          margin: 3em;
      }
      
      line, polyline { 
      align-self: baseline;
          fill: none;
          
      stroke: black;
      }
    rect { 
      align-self: baseline;
      border-end-end-radius: 80px 80px;
      fill:white;
          stroke: white;
      }
  </style>
  <div>
    <!-- For multiple genes, I would space them out a bit closer, and only repeat motifs
    one time-->
    <svg width="1350" height="300">
     
      
      {#each bondData as Array, i}
      <line x1={Array[0]*mult-parseFloat(Array[5])/5} x2={Array[2]*mult-parseFloat(Array[5])/5} y1={Array[1]*mult+parseFloat(Array[5])/5} y2={Array[3]*mult+parseFloat(Array[5])/5}></line>
      <line x1={parseFloat(Array[0])*mult+parseFloat(Array[5])/1.5} x2={parseFloat(Array[2])*mult+parseFloat(Array[5])/1.5} y1={parseFloat(Array[1])*mult-parseFloat(Array[5])/1.5} y2={parseFloat(Array[3])*mult-parseFloat(Array[5])/1.5}></line>
      {/each}
      {#each coordData as Array,i}
      <rect x={parseFloat(Array[0])*mult-7+parseFloat(Array[3])} y={parseFloat(Array[1])*mult-8+parseFloat(Array[3])} width=15 height="15" rx="3" ry="3"></rect>
      <text x={parseFloat(Array[0])*mult-5} y={parseFloat(Array[1])*mult+5}> {Array[2]} </text>
      {/each}
    </svg>
  </div>