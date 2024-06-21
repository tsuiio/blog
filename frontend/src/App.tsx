import { RouterProvider, createBrowserRouter } from "react-router-dom";
import "@/App.css";
import Layout from "@/pages/layout";
import Home from "@/pages/home";
import About from "./pages/about";

const router = createBrowserRouter([
  {
    id: "root",
    path: "/",
    element: <Layout />,
    children: [
      {
        index: true,
        element: <Home />,
      },
      {
        path: "/about",
        element: <About />,
      }
    ],
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;
