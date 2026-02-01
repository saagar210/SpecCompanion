# PROMPTS.md

This file contains step-by-step prompts that the user will paste into Claude Code. Each phase should be a self-contained unit of work with clear verification steps.

## Phase 1: Project Setup

### Prompt
```
Initialize a new Tauri project for Spec Companion desktop application. Use Rust as the backend with a web-based frontend (React/TypeScript). Set up the basic project structure with:
- Cargo.toml with proper dependencies (tauri, serde, sqlite, git2)
- Basic Tauri configuration in tauri.conf.json
- React frontend with TypeScript
- Project metadata in package.json
- Basic "Hello World" functionality to verify setup works

Follow Tauri conventions from CLAUDE.md. Create files in the following structure:
- src-tauri/Cargo.toml
- src-tauri/tauri.conf.json
- src-tauri/src/main.rs
- src/main.tsx
- src/index.html
- package.json

Verify the setup by running the app and ensuring it displays a basic "Hello World" message.
```

### What You Should Have After
- [ ] Tauri project initialized with Rust backend
- [ ] React/TypeScript frontend configured
- [ ] Basic project structure with all required files
- [ ] Working "Hello World" application

### Verification Command
```bash
cd src-tauri && cargo tauri build
```

### Common Issues
- If build fails due to missing dependencies, try `cargo update`
- If frontend doesn't load, verify `package.json` has correct dependencies

## Phase 2: Core Project Management

### Prompt
```
Implement core project management functionality for Spec Companion. Create the following features:

1. Project creation and management:
   - Create new projects with spec and codebase paths
   - Store project metadata locally
   - List existing projects
   - Project persistence with local storage

2. File system operations:
   - Upload SPEC.md and requirements files
   - Connect to codebase directories
   - Validate file paths
   - Handle file system errors gracefully

3. Basic project structure:
   - Create project directory structure
   - Store project metadata in SQLite database
   - Implement basic CRUD operations for projects

Use SQLite for local data storage. Follow the data models from SPEC.md. Create necessary database tables and implement basic operations.

See CLAUDE.md for Tauri backend conventions.
```

### What You Should Have After
- [ ] SQLite database schema for projects
- [ ] Project creation and management functions
- [ ] File system operations for spec/codebase handling
- [ ] Basic project listing UI

### Verification Command
```bash
# Test project creation
curl -X POST http://localhost:3000/api/projects -H "Content-Type: application/json" -d '{"name":"Test Project","specs":["spec1.md"],"codebase_path":"/path/to/codebase"}'
```

### Common Issues
- If database operations fail, verify SQLite setup and permissions
- If file paths don't resolve, ensure proper path handling for different OS

## Phase 3: Spec Document Handling

### Prompt
```
Implement spec document handling functionality for Spec Companion. Create the following features:

1. Spec parsing engine:
   - Parse SPEC.md files using pulldown-cmark
   - Extract sections, requirements, and testable elements
   - Support for markdown formatting
   - Handle multiple spec files

2. Spec analysis:
   - Extract requirements from markdown content
   - Categorize requirements by type
   - Generate spec metadata
   - Store parsed spec data in database

3. Spec management:
   - Upload and store spec files
   - Associate specs with projects
   - Retrieve and display spec content
   - Handle spec file validation

Implement the spec analysis engine that can process markdown files and extract structured requirements. Use the data models from SPEC.md for storage.

See CLAUDE.md for Tauri backend conventions and data model guidelines.
```

### What You Should Have After
- [ ] Spec parsing functionality using pulldown-cmark
- [ ] Requirement extraction and categorization
- [ ] Spec storage and retrieval system
- [ ] Basic spec display UI

### Verification Command
```bash
# Test spec parsing
curl -X POST http://localhost:3000/api/projects/1/specs -H "Content-Type: application/json" -d '{"content":"# Requirement 1\nThis is a test requirement"}'
```

### Common Issues
- If markdown parsing fails, verify pulldown-cmark version compatibility
- If requirements aren't extracted properly, check parsing logic

## Phase 4: Test Generation Engine

### Prompt
```
Implement the test generation engine for Spec Companion. Create the following features:

1. Test generation framework:
   - Framework-agnostic engine for generating tests
   - Support for multiple testing frameworks (JAST, PyTest, etc.)
   - Template-based generation system
   - Language-specific test file creation

2. Code analysis:
   - Analyze codebase to find relevant functions/classes
   - Match requirements to code elements
   - Generate test stubs for matching code
   - Handle different programming languages

3. Test file management:
   - Create test files in appropriate directories
   - Generate test code based on requirements
   - Store generated test metadata
   - Handle test file conflicts

Implement the core test generation logic that can take parsed requirements and generate appropriate test code. Follow the data models from SPEC.md for storing generated tests.

See CLAUDE.md for Tauri backend conventions and test generation guidelines.
```

### What You Should Have After
- [ ] Test generation engine with framework support
- [ ] Code analysis and matching functionality
- [ ] Test file creation and management
- [ ] Generated test storage system

### Verification Command
```bash
# Test test generation
curl -X POST http://localhost:3000/api/projects/1/generate-tests -H "Content-Type: application/json" -d '{"requirement_id":"req1"}'
```

### Common Issues
- If test generation fails, verify framework templates are properly configured
- If code analysis doesn't match requirements, check matching logic

## Phase 5: Test Execution Engine

### Prompt
```
Implement the test execution engine for Spec Companion. Create the following features:

1. Test execution framework:
   - Execute generated tests using appropriate test runners
   - Handle different testing frameworks (JAST, PyTest, etc.)
   - Capture test execution results
   - Manage test process lifecycle

2. Result processing:
   - Parse test execution output
   - Store test results in database
   - Generate execution reports
   - Handle test failures and errors

3. Integration with existing test runners:
   - Execute tests in isolated environments
   - Handle test timeouts and resource limits
   - Provide execution status updates

Implement the core test execution functionality that can run generated tests and collect results. Use the data models from SPEC.md for storing execution results.

See CLAUDE.md for Tauri backend conventions and test execution guidelines.
```

### What You Should Have After
- [ ] Test execution framework with multiple framework support
- [ ] Result processing and storage system
- [ ] Test runner integration
- [ ] Execution status tracking

### Verification Command
```bash
# Test test execution
curl -X POST http://localhost:3000/api/projects/1/execute-tests -H "Content-Type: application/json" -d '{"test_ids":["test1","test2"]}'
```

### Common Issues
- If test execution fails, verify test runner paths and permissions
- If results aren't captured, check output parsing logic

## Phase 6: Reporting & UI

### Prompt
```
Implement rich reporting and user interface for Spec Companion. Create the following features:

1. Reporting dashboard:
   - Display spec-to-code alignment reports
   - Show coverage statistics
   - Visualize mismatches between specs and code
   - Export reports in various formats

2. Interactive UI components:
   - Project listing and management
   - Spec viewing and editing
   - Test execution status
   - Alignment visualization

3. Data visualization:
   - Coverage percentage charts
   - Mismatch reporting tables
   - Requirement traceability
   - Performance metrics

4. Export functionality:
   - Export alignment reports
   - Export test results
   - Export spec documents

Create a complete, user-friendly interface that displays all project information and test results. Follow React/TypeScript best practices and UI/UX guidelines.

See CLAUDE.md for UI/UX conventions and data visualization guidelines.
```

### What You Should Have After
- [ ] Complete reporting dashboard UI
- [ ] Interactive project management interface
- [ ] Spec viewing and editing components
- [ ] Test execution status display
- [ ] Export functionality for reports

### Verification Command
```bash
# Verify UI loads correctly
cd src && npm start
```

### Common Issues
- If UI components don't render, verify React component structure
- If data doesn't display, check API endpoint connections

## Phase 7: Advanced Features

### Prompt
```
Implement advanced features for Spec Companion. Create the following functionality:

1. CI/CD integration:
   - Automated spec alignment checking
   - Integration with popular CI platforms
   - Webhook support for automated execution
   - Status reporting to CI systems

2. Git integration:
   - Version control integration
   - Conflict detection and resolution
   - Commit hooks for automatic alignment checking
   - Branch-based analysis

3. Performance optimization:
   - Incremental analysis for large codebases
   - Caching mechanisms for repeated operations
   - Memory usage optimization
   - Parallel processing capabilities

4. Plugin architecture:
   - Support for custom test generators
   - Extensible framework for new features
   - Plugin installation and management

Implement these advanced features to make Spec Companion production-ready and extensible. Follow Tauri conventions for integration and performance optimization.

See CLAUDE.md for advanced feature implementation guidelines.
```

### What You Should Have After
- [ ] CI/CD integration capabilities
- [ ] Git integration with conflict detection
- [ ] Performance optimization features
- [ ] Plugin architecture support
- [ ] Automated execution workflows

### Verification Command
```bash
# Test advanced features
curl -X POST http://localhost:3000/api/projects/1/analyze -H "Content-Type: application/json" -d '{"incremental":true}'
```

### Common Issues
- If integration fails, verify API endpoints and permissions
- If performance is poor, check caching and optimization logic