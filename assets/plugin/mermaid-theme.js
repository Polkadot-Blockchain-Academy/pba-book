// Mermaid theme and functions
const pink = "#E6007A";
const purple = "#442299";
const lightPurple = "#6D3AEE";
const white = "#FFFFFF";

Reveal.addEventListener("ready", event => asyncMermaidRender(event));

async function asyncMermaidRender(event) {
  var graphs = document.querySelectorAll(".mermaid");
  graphs.forEach((item, index) => {
    let graphCode = item.textContent.trim(); // trim() becase of gantt, class and git diagram
    let mermaidDiv = document.createElement("div");
    mermaidDiv.classList.add("mermaid");
    mermaidDiv.setAttribute("data-processed", "true");

    try {
      // item.innerText ignores html elements added by revealjs highlight plugin.
      mermaid.mermaidAPI.render("theGraph" + index, graphCode, function(svgCode) {
        mermaidDiv.innerHTML = svgCode;
      });

      let parentDiv = document.createElement("div");
      parentDiv.appendChild(mermaidDiv);
      parentDiv.style.width = "100%";
      parentDiv.style.height = "100%";
      item.parentNode.parentNode.insertBefore(parentDiv, item.parentNode);
      item.parentNode.remove();
    } catch (err) {
      console.log("Cannot render mermaid diagram " + index + "\n" + graphCode);
      console.log(err.message);
    }
  });
}
