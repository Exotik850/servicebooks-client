export function displayObject(obj, indent = 0) {
  const inc = 10;
  if (obj && typeof obj === "object") {
      return Object.entries(obj)
          .map(([key, value]) => {
              if (typeof value === "object") {
                  return `<div class="nested" style="padding-left:${
                      indent + inc
                  }px">
                  <p><strong>${key}:</strong></p>
                  ${displayObject(value, indent + inc)}  
              </div>`;
              } else {
                  return `<p style="padding-left:${indent}px"><strong>${key}:</strong> ${value}</p>`;
              }
          })
          .join("");
  } else if (obj !== undefined && obj !== null) {
      return `<p style="padding-left:${indent}px">${obj}</p>`;
  } else {
      return "<br/>";
  }
}