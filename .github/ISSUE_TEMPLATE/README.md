# GitScribe Issue Templates

This directory contains GitHub issue templates for the GitScribe project. These templates help maintain consistency and ensure all necessary information is provided when creating issues.

## Available Templates

### 🔧 Task Implementation (`task-implementation.yml`)
Use this template when creating issues for implementing specific tasks from our project breakdown (`project-tasks.md`).

**When to use:**
- Implementing a specific task from the project roadmap
- Breaking down large tasks into smaller implementation issues
- Tracking progress on project milestones

**Key features:**
- Links directly to task IDs from `project-tasks.md`
- Tracks dependencies and parallel execution opportunities
- Includes acceptance criteria and deliverables
- Estimates implementation effort

### 🐛 Bug Report (`bug_report.yml`)
Use this template to report bugs, crashes, or unexpected behavior.

**When to use:**
- Application crashes or errors
- Features not working as expected
- Performance issues
- Data corruption or loss

**Key features:**
- Component-specific categorization
- Severity levels for prioritization
- Environment information collection
- Step-by-step reproduction instructions

### ✨ Feature Request (`feature_request.yml`)
Use this template to suggest new features or enhancements.

**When to use:**
- Proposing new functionality
- Suggesting improvements to existing features
- Requesting plugin or theme capabilities
- Architectural enhancements

**Key features:**
- Problem statement and solution description
- Use case documentation
- Implementation complexity assessment
- Relationship to existing project tasks

### ❓ Question / Discussion (`question.yml`)
Use this template for questions, discussions, or general inquiries.

**When to use:**
- Technical implementation questions
- Clarification on project direction
- Contributing guidelines questions
- General support requests

**Key features:**
- Categorized question types
- Context and environment information
- Links to relevant documentation

## Using the Templates

### For Task Implementation

1. **Review `project-tasks.md`** first to understand the task structure and dependencies
2. **Choose the correct Task ID** (e.g., T1.1, T2.3, T6.5)
3. **Check dependencies** to ensure prerequisite tasks are complete
4. **Consider parallel execution** opportunities with other tasks
5. **Fill out all required fields** including acceptance criteria

### For Bug Reports

1. **Search existing issues** to avoid duplicates
2. **Identify the affected component** (CLI, GUI, etc.)
3. **Assess severity** honestly to help with prioritization
4. **Provide clear reproduction steps** - this is crucial for fixing bugs
5. **Include environment details** and relevant logs

### For Feature Requests

1. **Check project requirements** to see if the feature is already planned
2. **Review existing tasks** to understand if it fits into current roadmap
3. **Describe the problem** you're trying to solve, not just the solution
4. **Consider plugin alternatives** - many features can be implemented as plugins
5. **Provide use cases** to help prioritize the request

## Labels and Organization

### Automatic Labels
- **Task Implementation:** `task`, `development`
- **Bug Report:** `bug`, `needs-triage`
- **Feature Request:** `enhancement`, `needs-triage`
- **Question:** `question`, `discussion`

### Additional Labels (Applied by Maintainers)
- **Priority:** `priority-critical`, `priority-high`, `priority-medium`, `priority-low`
- **Component:** `cli`, `gui`, `plugin-system`, `build-system`, etc.
- **Status:** `in-progress`, `blocked`, `ready-for-review`, `completed`
- **Type:** `breaking-change`, `documentation`, `performance`, `security`

## Project Integration

These templates are designed to work seamlessly with our project structure:

- **`project-tasks.md`** - Complete task breakdown and dependencies
- **`project-requirement.md`** - Project requirements and specifications
- **`project-discussion.md`** - Technical design decisions

## Best Practices

### For Contributors
1. **Use the most specific template** for your issue type
2. **Fill out all required fields** completely
3. **Reference related issues** and tasks when applicable
4. **Update issues** as work progresses
5. **Close issues** when work is complete

### For Maintainers
1. **Triage new issues** within 48 hours
2. **Apply appropriate labels** for organization
3. **Link issues to project milestones** when applicable
4. **Update task dependencies** as issues are completed
5. **Maintain the project board** with current status

## Template Customization

These templates can be modified as the project evolves:

1. **Add new components** to dropdown lists as the codebase grows
2. **Update task references** when the project structure changes
3. **Modify priority levels** based on project needs
4. **Add new template types** for specific workflows

## Getting Help

If you're unsure which template to use or need help with the issue creation process:

1. Check the **project documentation** first
2. Look at **existing issues** for examples
3. Start a **discussion** for general questions
4. Contact **maintainers** for guidance

---

*These templates are designed to streamline the development process and ensure consistent communication across the GitScribe project.* 