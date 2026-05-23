import { createHashRouter, RouterProvider } from "react-router-dom";
import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap/dist/js/bootstrap.bundle.min.js";
import "./css/style.css";
import { Page } from "./components/Page";
import { IndexPage } from "./pages";
import { DetailPage } from "./pages/detail";

const router = createHashRouter([
    {
      path: "/",
      element: <Page><IndexPage /></Page>,
    },
    {
      path: "/detail",
      element: <Page><DetailPage /></Page>,
    }
  ]);
function App() {
  return (
    <RouterProvider router={router} />
  );
}

export default App;
