export type Genes = {
  length: number,
  genes: [[number]]
};


// Fetches a random collection of genes from the API
export function getGenes(): Promise<Genes> {
  return fetch("http://localhost:8080/gene").then((resp) => resp.json()).catch((err) => console.error(err));
}
