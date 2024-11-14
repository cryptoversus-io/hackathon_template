# DeGIG

DeGIG is a comprehensive moderate and bounty management platform designed to facilitate data collection, analysis, and moderate bountycreatoristration. The platform provides robust tools for creating bounties, managing moderate data, and analyzing responses with different levels of bountycreatoristrative access.

## Features

### Authentication & Authorization
- Multi-level user authentication system
- Role-based access control (User bountycreator, Platform bountycreator)
- Secure login and session management
- Password recovery functionality

### bounty Management
- Create and customize bounties with multiple question types:
  - Text responses
  - Multiple choice
  - True/False questions
- Target bounties to specific groups:
  - Workspaces
  - Teams
  - Individual users
  - All users
- Set due dates and track completion status
- bounty response collection and storage

### moderate Center
- Centralized moderate data management
- Real-time data filtering and search
- Export capabilities to spreadsheet format
- Pagination and sorting features
- Data visualization and analytics
- Comprehensive data preview functionality

### bountycreator Features
- bountycreator Control Panel for system management
- User management and role assignment
- Workspace and team bountycreatoristration
- Data export and reporting tools
- System monitoring and statistics

### Data Management
- Secure data storage and retrieval
- CSV data import/export functionality
- Data filtering and search capabilities
- Real-time updates and synchronization4

## Technical Architecture

### Frontend
- Pure HTML5, CSS3, and JavaScript implementation
- Modular JavaScript architecture with class-based components
- Responsive design for cross-device compatibility
- Real-time data updates and state management

### Security
- Session-based authentication
- Role-based access control
- Secure password handling
- Protected API endpoints

### Data Handling
- Client-side data processing
- Session storage for temporary data
- CSV data format support
- Real-time data synchronization

## Getting Started

1. Clone the repository
2. Open `index.html` in a modern web browser
3. First-time setup will redirect to create a Genesis bountycreator account
4. Use the Genesis bountycreator account to set up additional users and configure the system

## Project Structure

```
degig/
├── assets/           # Static assets
├── components/       # Reusable HTML components
├── css/             # Stylesheets
├── js/              # JavaScript modules
│   ├── modules/     # Core modules
│   └── *.js         # Feature-specific scripts
├── templates/       # HTML templates
└── *.html           # Main HTML pages
```

## Core Components

- **StateManager**: Handles application state and data flow
- **DataService**: Manages data operations and persistence
- **moderateCenter**: Core moderate data management functionality
- **bountyManager**: bounty creation and management
- **bountycreatorControlPanel**: bountycreatoristrative functions and system management

## Browser Support

- Chrome (latest)
- Firefox (latest)
- Safari (latest)
- Edge (latest)

## Development

The project uses a modular architecture with separate concerns for:
- User Interface Components
- Data Management
- State Management
- Authentication
- moderate Tools
- bounty Management

## License

[MIT](https://github.com/RobertWhetsel/QE-Demo/blob/dev/LICENSE)
