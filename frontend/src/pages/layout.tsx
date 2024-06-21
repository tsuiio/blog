import { Outlet } from "react-router-dom";

export default function Layout() {
  return (
    <div className="flex flex-col min-h-[100vh]">
      <Header />
    <div className="flex-1">
      <Outlet />
      </div>
      <Footer />
    </div>
  );
}

function Header() {
  return (
    <>
      <p>test</p>
    </>
  );
}

function Footer() {
  return (
    <div className="text-center">
      <p>foo</p>
    </div>
  );
}
