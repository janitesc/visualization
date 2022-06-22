<script lang="ts">

  // TODO: implement GeneViewer by calling into the /genes api endpoint to get data
  // TODO: add a MolViewer along the same lines
  import GeneViewer from "./components/GeneViewer.svelte";
  import { getGenes, Genes } from "./api";
  import { onMount } from "svelte";
  let lengthdata;
  let genesdata = [];
  onMount(async () => {
  const genes = getGenes();
  await getGenes();
  //for multiple genes, I would make another array and make an
  //array of genes
  genes.then((data) => {
    lengthdata = data.length;
    genesdata = data.genes;
  })
});
</script>

<main>
  GENEVIEWER
  <!-- For multiple genes, I would get the maximum length
  of the gene array, and use that one to scale the x-axis, so the 
  graph was neat and easy to read.  -->
  <div>
    <GeneViewer length = {lengthdata} genedata = {genesdata} />
    
  </div>
  <div id="last">
    <svg width="1500" height="100">
      <line x1="50" x2="1350" y1="10" y2="10"></line>
      <text x="50" y = "0">0</text>
      <text x="180" y = "0">{Math.round(lengthdata/10)}</text>
        <text x="310" y = "0">{Math.round((lengthdata/10))*2}</text>
        <text x="440" y = "0">{Math.round((lengthdata/10))*3}</text>
        <text x="570" y = "0">{Math.round((lengthdata/10))*4}</text>
        <text x="700" y = "0">{Math.round((lengthdata/10))*5}</text>
        <text x="830" y = "0">{Math.round((lengthdata/10))*6}</text>
        <text x="960" y = "0">{Math.round((lengthdata/10))*7}</text>
        <text x="1090" y = "0">{Math.round((lengthdata/10))*8}</text>
        <text x="1220" y = "0">{Math.round((lengthdata/10))*9}</text>
        <text x="1350" y = "0">{lengthdata}</text>
      <g class="x">
       
        
      </g> 
      </svg>
  </div>
 Length: {lengthdata}

</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    margin: 0 auto;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
  }

  div {
    display: flex;
    justify-content: center;
    align-items: center;
  }

  #last {
    

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
	
	.x text {
		text-anchor: left;
	}
</style>
