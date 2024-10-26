import {Nav, Navbar } from "react-bootstrap";

function Header() {
  return (
    <Navbar expand="lg" variant="">
      <Navbar.Brand href="/">Nathan Cobb</Navbar.Brand>
      <Nav className="me-auto">
        <Nav.Link>Most Recent Post</Nav.Link>
      </Nav>
    </Navbar>
  );
}
export default Header;
