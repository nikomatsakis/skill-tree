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

function convertDivToSkillTree(divId, dotText) {
  viz.renderSVGElement(dotText).then(element => {
    document.getElementById(divId).appendChild(element);
  })
}