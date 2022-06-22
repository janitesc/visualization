<script lang="ts">
  import { getGenes, Genes } from "../api.ts";
  export let length: number;
  export let genedata = [];
  debugger;
  console.log(genedata);
  console.debug(getGenes());
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
		stroke: rgb(0, 0, 0);
	}
</style>




<div>
  <svg width="1500" height="300">
    <line x1="100" x2=1400 y1="50" y2="50"></line>
    {#each genedata as Array,i}
    <rect x={[(Array[0]/length)*1300+100]} y="0" width={[(Array[1]/length)*1300+100]-[[(Array[0]/length)*1300+100]]} height="100" rx="10" ry="10" id="Motif [i]"style="fill:{[colors[i]]};" />
    <rect x="1400" y={[(Array[0]/length)*400+20]} width={10} height="10" rx="5" ry="5" id="Motif [i]" style="fill:{[colors[i]]};">
      <title>Motif {i+1}</title>
    </rect> />
    <text x="1415" y={[(Array[0]/length)*400+30]}>Motif {i+1} </text>
    {/each}
  </svg>
</div>