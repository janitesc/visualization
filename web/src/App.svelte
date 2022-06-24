<script lang="ts">

  // TODO: implement GeneViewer by calling into the /genes api endpoint to get data
  // TODO: add a MolViewer along the same lines
  import GeneViewer from "./components/GeneViewer.svelte";
  import { getGenes, Genes } from "./api";
  import { getmol, mol } from "./api";
  import { onMount } from "svelte";
  let lengthdata;
  let genesdata = [];
  let Coorddata = [];
  let bonddata = [];
  debugger;
  onMount(async () => {
  const mol = getmol();
  const genes = getGenes();
  debugger;
  await getGenes();
  await getmol();
  //for multiple genes, I would make another array and make an
  //array of genes
  genes.then((data) => {
    lengthdata = data.length;
    genesdata = data.genes;
    console.log(lengthdata[0]);
  })
  mol.then((data) => {
    Coorddata = data.coordData;
    bonddata = data.bonddata;
    console.log(Coorddata[0]);
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
    <svg width="1350" height="100">
      <line x1="25" x2="1225" y1="10" y2="10"></line>
      <text x="25" y = "0">0</text>
      <text x="145" y = "0">{Math.round(lengthdata/10)}</text>
        <text x="265" y = "0">{Math.round((lengthdata/10))*2}</text>
        <text x="385" y = "0">{Math.round((lengthdata/10))*3}</text>
        <text x="505" y = "0">{Math.round((lengthdata/10))*4}</text>
        <text x="625" y = "0">{Math.round((lengthdata/10))*5}</text>
        <text x="745" y = "0">{Math.round((lengthdata/10))*6}</text>
        <text x="865" y = "0">{Math.round((lengthdata/10))*7}</text>
        <text x="985" y = "0">{Math.round((lengthdata/10))*8}</text>
        <text x="1105" y = "0">{Math.round((lengthdata/10))*9}</text>
        <text x="1225" y = "0">{lengthdata}</text>
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
  html, body {
                width: 100%;
                height: 100%;
                margin: 0;
                padding: 0;
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
