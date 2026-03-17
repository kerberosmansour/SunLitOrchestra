interface SidebarProps {
  onNewSession: () => void;
  onSelectSettings: () => void;
}

function Sidebar({ onNewSession, onSelectSettings }: SidebarProps) {
  return (
    <nav className="sidebar">
      <div className="sidebarBrand">
        <img
          src="/sunlit.jpeg"
          alt="SunLit logo"
          className="sidebarBrandLogo"
        />
        <span className="sidebarTitle">SunLit</span>
      </div>

      <div className="sidebarNav">
        <div className="sidebarSection">
          <button
            className="sidebarLink"
            onClick={onNewSession}
            aria-label="New Session"
          >
            <span className="sidebarIcon">✨</span>
            <span className="sidebarLabel">New Session</span>
          </button>
        </div>

        <div className="sidebarSection">
          <span className="sidebarSectionLabel">Sessions</span>
          <ul className="sidebarSectionList">
            <li>
              <span className="sidebarLink" style={{ opacity: 0.5 }}>
                <span className="sidebarIcon">💬</span>
                <span className="sidebarLabel">No sessions yet</span>
              </span>
            </li>
          </ul>
        </div>
      </div>

      <div className="sidebarFooter">
        <button
          className="sidebarLink"
          onClick={onSelectSettings}
          aria-label="Settings"
        >
          <span className="sidebarIcon">⚙️</span>
          <span className="sidebarLabel">Settings</span>
        </button>
      </div>
    </nav>
  );
}

export default Sidebar;
