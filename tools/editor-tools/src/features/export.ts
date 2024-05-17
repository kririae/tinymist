import van from "vanjs-core";
const { div } = van.tags;

export const Export = () => {
  // #tinymist-app.no-wrap
  document.getElementById("tinymist-app")?.classList.add("no-wrap");

  const ww = (content: string) =>
    div(
      {
        style:
          "width: (100% - 20px); aspect-ratio: 1 / 1.422; border: 1px solid white; padding: 10px; margin: 8px 0;",
      },
      content
    );

  return div(
    {
      class: "flex flex-row",
      style: "height: 100vh; overflow: hidden",
    },
    div(
      {
        style:
          "flex: 6; height: 100%; padding: 10px; overflow-x: hidden; overflow-y: scroll",
      },
      ww("I'm page 1"),
      ww("I'm page 2"),
      div({
        style:
          "width: (100% - 20px); height: 1px; padding: 0 10px; margin: 7px 0;",
      })
    ),
    div(
      {
        style: "flex: 4; height: 100%; overflow-x: hidden; overflow-y: scroll",
      },
      "SVG/PNG/PDF",
      "ToPath",
      "PDF Identifier",
      "PDF Timestamp",
      "PPI",
      "Pages",
      "Merge Pages",
      "Frame Fill",
      "Merge Pages / Padding",
      "Merge Pages / Padding Fill"
    )
  );
};
