// Loads the dot file found at `dot_path` as text and displays it.
function loadSkillTree(dot_path) {
  var viz = new Viz();
  fetch(dot_path)
    .then(response => response.text())
    .then(text => {
      viz.renderSVGElement(text)
        .then(element => { document.body.appendChild(element); })
    });
}

function setSvgSizeAndViewBox(svgElem) {
  // Same as --content-max-width
  const defaultWidth = "750px";
  const defaultHeight = "500px";
  // Make the element have a fixed size, but use the width/height
  // to set the viewBox
  let width = svgElem.getAttribute("width").slice(0, -2);
  let height = svgElem.getAttribute("height").slice(0, -2);
  let viewBox = "0 0 " + width + " " + height;
  svgElem.setAttribute("width", defaultWidth);
  svgElem.setAttribute("height", defaultHeight);
  svgElem.setAttribute("viewBox", viewBox);
  svgElem.setAttribute("class", "skills-tree");
}

function appendControls(parent, svgElem, pz) {
  const pzXform = pz.getTransform();;
  const initialZoom = {x: pzXform.x, y: pzXform.y, scale: pzXform.scale};
  const controls = document.createElement("div");
  controls.setAttribute("style", "padding-bottom: 4px");

  const controlsText = document.createElement("i");
  controlsText.setAttribute("style", "font-size: smaller");
  controlsText.innerText = "Hold the alt/option key to zoom the skill tree; click and move to pan.";

  const resetButton = document.createElement("button");
  resetButton.setAttribute("title", "Reset");
  resetButton.setAttribute("aria-label", "Reset pan/zoom");
  resetButton.innerHTML = '<i class="fa fa-window-maximize" />'
  resetButton.appendChild(document.createTextNode(" Reset Pan/Zoom"));
  resetButton.onclick = (e) => {
    pz.moveTo(initialZoom.x, initialZoom.y);
    pz.zoomAbs(initialZoom.x, initialZoom.y, initialZoom.scale);
  }
  controls.appendChild(controlsText);
  controls.appendChild(resetButton);
  controls.className = 'buttons';
  parent.insertBefore(controls, svgElem);
}

function convertDivToSkillTree(divId, dotText) {
  new Viz().renderSVGElement(dotText.dot_text).then(svg_elem => {
    let parent = document.getElementById(divId);
    parent.appendChild(svg_elem);
    setSvgSizeAndViewBox(svg_elem);
    let element = svg_elem.children[0];
    let pz = panzoom(element, {
      bounds: true,
      boundsPadding: 0.1,
      beforeWheel: function(e) {
        // allow wheel-zoom only if altKey is down. Otherwise - ignore
        var shouldIgnore = !e.altKey;
        return shouldIgnore;
      }
    });
    appendControls(parent, svg_elem, pz)
  })
}

for (let obj of SKILL_TREES) {
  convertDivToSkillTree(obj.id, obj.value);
}
