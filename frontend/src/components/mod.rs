// Insert thought
mod bottom;

// Top Left header area
mod topleft;

// Topright header area
mod topright;

// Info sidebar
mod rightsidebar;

// Tag/filter/search sidebar
mod leftsidebar;

//
mod center;

mod headerbar;

pub use {
    bottom::Bottom, center::Center, headerbar::HeaderBar, leftsidebar::LeftSideBar,
    rightsidebar::RightSideBar, topleft::TopLeft, topright::TopRight,
};
