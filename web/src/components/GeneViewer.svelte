<script lang="ts">
  import { getGenes, Genes } from "../api.ts";
import App from "../App.svelte";
  export let length: number;
  export let genedata = [];
  debugger;
  console.log(genedata);
  console.debug(getGenes());
  //In a bigger group of genes, I would autogenerate this and call however
  //many colors were needed, but a smaller sample meant
  //I could pick the colors, leaving with 20 unique modules
  //that can be seen
  let colors = ["Lavender", "Plum", "Orchid","Fuchsia","MediumOrchid","DarkOrchid","BlueViolet","Purple","MediumSlateBlue","DarkSlateBlue","RebeccaPurple","Indigo","MidnightBlue", "MediumBlue","RoyalBlue", "DeepSkyBlue", "CornflowerBlue","SkyBlue", "PowderBlue","LightSteelBlue"];
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
    fill:Aquamarine;
		stroke: rgb(0, 0, 0);
	}
</style>
<div>
  <!-- For multiple genes, I would space them out a bit closer, and only repeat motifs
  one time-->
  <svg width="1500" height="300">
    <line x1="50" x2=1350 y1="50" y2="50"></line>
    {#each genedata as Array,i}
    <rect x={[(Array[0]/length)*1300+50]} y="0" width={[(Array[1]/length)*1300+100]-[[(Array[0]/length)*1300+100]]} height="100" rx="10" ry="10" id="Motif [i]"style="fill:{[colors[i]]};" />
    <rect x="1365" y={i*30} width={15} height="15" rx="3" ry="3" id="Motif [i]" style="fill:{[colors[i]]};">
      <title>Motif {i+1}</title>
    </rect> />
    <text x="1384" y={i*30 + 13}>Motif {i+1} </text>
    {/each}
  </svg>
</div>