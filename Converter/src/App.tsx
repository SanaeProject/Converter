import { createHashRouter, RouterProvider } from "react-router-dom";
import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap/dist/js/bootstrap.bundle.min.js";
import "./css/style.css";
import { Page } from "./components/Page";
import { IndexPage } from "./pages";

const router = createHashRouter([
    {
      path: "/",
      element: <Page><IndexPage /></Page>,
    }
  ]);
function App() {
  return (
    <RouterProvider router={router} />
  );
}

export default App;
