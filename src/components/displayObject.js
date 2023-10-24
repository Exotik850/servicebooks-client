// export function displayObject(obj) {
//   if (obj && typeof obj === "object") {
//     let table = `<table style="max-width: 95%">`;
//     Object.entries(obj).forEach(([key, value]) => {
//       if (typeof value === "object") {
//         table += `
//           <tr>
//             <td><strong>${key}:</strong></td>
//             <td>
//               ${displayObject(value)}
//             </td>
//           </tr>
//         `;
//       } else {
//         table += `
//           <tr>
//             <td><strong>${key}:</strong></td>
//             <td>${value}</td>
//           </tr>
//         `;
//       }
//     });
//     table += `</table>`;
//     return table;
//   } else if (obj !== undefined && obj !== null) {
//     return `<p>${obj}</p>`;
//   } else {
//     return "<br/>";
//   }
// }

export function displayObject(obj, indent = 0) {
  const inc = 10;
  if (obj && typeof obj === "object") {
    return Object.entries(obj)
      .map(([key, value]) => {
        if (typeof value === "object") {
          return `<div class="nested" style="padding-left:${indent}px">
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
