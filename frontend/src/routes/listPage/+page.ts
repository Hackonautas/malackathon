// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true;

export async function load() {
  const res = await fetch("http://localhost:3000/reservoirs/0");
  const embalses = await res.json();
  console.log(embalses);
  return {
    embalses: embalses,
  };
}
