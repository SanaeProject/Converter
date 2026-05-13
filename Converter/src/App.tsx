import { createHashRouter, RouterProvider } from "react-router-dom";
import "./lib/bootstrap-5.3.8-dist/css/bootstrap.min.css";

const router = createHashRouter([
    {
      path: "/",
      element: <h1 className="text-center mt-5">Hello, World!</h1>,
    } 
  ]);
function App() {
  return (
    <RouterProvider router={router} />
  );
}

export default App;
