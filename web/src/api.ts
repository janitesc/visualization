export type Genes = {
  length: number,
  genes: [[number]]
};


// Fetches a random collection of genes from the API
export function getGenes(): Promise<Genes> {
  return fetch("http://localhost:8080/gene").then((resp) => resp.json()).catch((err) => console.error(err));
}

export type mol = {
  coordData: [[string]]
  //I want this to include: ID number, coordinate data, classification
  //coordinateData: [[string]],
  bonddata: [[string]]
};


// Fetches a random collection of genes from the API
export function getmol(): Promise<mol> {
  return fetch("http://localhost:8085/mol").then((resp) => resp.json()).catch((err) => console.error(err));
}
