<script lang="ts">
  // TODO: implement GeneViewer by calling into the /genes api endpoint to get data
  // TODO: add a MolViewer along the same lines
  import GeneViewer from "./components/GeneViewer.svelte";
</script>

<main>
  <GeneViewer width=200 height=200/>
  <div>
    <GeneViewer width=600 height=300/>
    <GeneViewer width=600 height=300/>
  </div>
  <div id="last">
    <GeneViewer width=1200 height=400/>
  </div>
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
    width: 900px;
  }
</style>
