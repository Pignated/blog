import "./index.css"
import Signup from './pages/signup';
import Header from "./components/header";
import Home from "./pages/home";
import Post from "./pages/post";
import {BrowserRouter, Route, Routes} from "react-router-dom"

function App() {
  return (
    <>
      <Header/>
      <BrowserRouter>
      <Routes>
        <Route path="/" exact element={<Home/>}/>
        <Route path="/posts/:postID" element={<Post/>}/>
        <Route path="/signup" element={<Signup/>}/>
      </Routes>
      </BrowserRouter>
    </>
  );
}

export default App;
