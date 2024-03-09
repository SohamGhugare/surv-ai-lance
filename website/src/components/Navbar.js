import React from 'react'
import "../stylesheets/prep.css"
import "../stylesheets/Navbar.css"

const Navbar = () => {
  return (
    <div className='navbarParent'>
      <div className='navbarContainer'>
        <h2 className='Logo'>Surv-<span className='Coloured'>AI</span>-lance</h2>
        <p className='navbarContents'>About</p>
        <p className='navbarContents'>Login</p>
      </div>
    </div>
  )
}
const navbar = document.querySelector('#NavBar');
let top = navbar.offsetTop;
function stickynavbar() {
  if (window.scrollY >= top) {    
    navbar.classList.add('sticky');
  } else {
    navbar.classList.remove('sticky');    
  }
}
window.addEventListener('scroll', stickynavbar);

export default Navbar