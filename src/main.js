window.addEventListener("DOMContentLoaded", () => {
  const connectBtn = document.getElementById("connect");

  connectBtn.addEventListener("click", () => {
    const name = document.getElementById("name").value.trim();
    const email = document.getElementById("email").value.trim();
    const status = document.getElementById("status");

    if (!name || !email) {
      status.innerText = "Please enter both your name and email.";
      return;
    }

    const win = window.open("", "_self"); // replaces current view

    win.document.write(`
      <html>
        <head>
          <style>
            body {
              margin: 0;
              height: 100vh;
              overflow: hidden;
            }
          </style>
        </head>
        <body>
          <script>
            window.Plain = {
              requireAuthentication: true,
              email: "${email}",
              fullName: "${name}"
            };
          </script>
          <script async src="https://app.plain.com/embed.js"></script>
        </body>
      </html>
    `);
  });
});