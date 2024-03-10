import React from 'react'
import { useState, useEffect } from 'react'
import "../stylesheets/prep.css"
import "../stylesheets/Navbar.css"

const Navbar = () => {
  const [navbarContainer, setNavbar] = useState(false);
  useEffect(() => {
    const changeBackground = () => {
      if (window.scrollY >= 5)
      {
        setNavbar(true);
      }
      else
      {
        setNavbar(false);
      }
    };
    window.addEventListener('scroll',changeBackground);
    return () => {
      window.removeEventListener('scroll',changeBackground);
    };
  }, []); 
  return (
    <div className='navbarParent'>
      <div className={navbarContainer ? 'navbarContainer active' : 'navbarContainer'}>
        <h2 className='Logo'>Surv-<span className='Coloured'>AI</span>-lance</h2>
        <p className='navbarContents'>About</p>
        <p className='navbarContents'>Login</p>
      </div>
    </div>
  )
};
export default Navbar;