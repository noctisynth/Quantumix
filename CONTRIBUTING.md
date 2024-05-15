# Contributing

Before reading this CONTRIBUTING file, please take note to read the [Code of Conduct](CODE_OF_CONDUCT.md).

## Naming Conventions

### Branch Naming Convention

Name branches according to their purpose:

1. **Feature Branch (feature)**:

   - Naming format: `feat/short-description` or `feat/issue-number-short-description`.
   - Examples: `feat/login-page`, `feat/123-add-authentication`

2. **Bugfix Branch (bugfix)**:

   - Naming format: `fix/short-description` or `fix/issue-number-short-description`.
   - Examples: `fix/login-error`, `fix/456-register-bug`

3. **Development Branch (development)**:

   - Typically named `dev` or `development`.

4. **Release Branch (release)**:

   - Naming format: `release/version-number`.
   - Example: `release/1.0.0`

5. **Hotfix Branch (hotfix)**:

   - Naming format: `hotfix/short-description` or `hotfix/issue-number-short-description`.
   - Examples: `hotfix/critical-login-issue`, `hotfix/789-critical-bug`

6. **Documentation Branch (docs)**:
   - Used for changes related to documentation.
   - Naming format: `docs/short-description`.
   - Example: `docs/update-readme`

### Commit Message Convention

Commit messages consist of three parts: header, body (optional), and footer (optional).

1. **Header**:

   - Format: `<type>(<scope>): <subject>`
   - `type` is the category of the commit (e.g., feat, fix, docs, style, refactor, test, chore).
   - `scope` is the module or feature affected (optional).
   - `subject` is a brief description, limited to 50 characters.

   Available types:

   - `feat`: A new feature.
   - `fix`: A bug fix.
   - `docs`: Documentation only changes.
   - `style`: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc).
   - `refactor`: A code change that neither fixes a bug nor adds a feature.
   - `test`: Adding missing or correcting existing tests.
   - `chore`: Changes to the build process or auxiliary tools and libraries such as documentation generation.
   - `perf`: A code change that improves performance.
   - `ci`: Changes to our CI configuration files and scripts.
   - `revert`: Reverts a previous commit.
   - `build`: Changes that affect the build system or external dependencies (example scopes: gulp, broccoli, npm).

2. **Body** (optional):

   - Provides more details, explaining the reasons and content of the change.
   - Each line should not exceed 72 characters.

3. **Footer** (optional):
   - Includes issue references or other information.

### Examples

#### Branch Naming Examples

```plaintext
feat/add-user-authentication
fix/login-error
release/1.2.0
hotfix/urgent-security-fix
docs/update-readme
```

#### Commit Message Examples

```plaintext
feat(auth): add user authentication

- Implemented login and register pages
- Integrated authentication API
- Added unit tests for authentication module

Resolves #123
```

```plaintext
fix(login): correct login error handling

- Fixed issue where incorrect password caused login failure without proper error message
- Updated error messages for better user feedback

Related to #456
```

```plaintext
release: 1.2.0

- Includes new features: user authentication, dashboard redesign
- Bug fixes: login error, UI issues on mobile devices
- Performance improvements

Reviewed by @fu050409
```

```plaintext
docs(readme): update installation instructions

- Added steps for setting up development environment
- Clarified dependency requirements

No related issues
```

## Maintainers & Collaborators

> Before carrying out the following operations, please make sure you have been invited by the **Narrator Council** and joined the Noctisynth organization's repository on GitHub.
>
> Make sure your branch name and commit message follow the naming conventions mentioned above.

1. Clone this repository to your local machine.
2. Create a new branch in your workspace:

   ```bash
   git checkout -b new_branch
   ```

3. Make revisions.
4. Save and push:

   ```bash
   git add *
   git commit -m "feat(scope): content"
   git push --set-upstream origin new_branch
   ```

5. Initiate a Pull Request.

## Outside Collaborators & New Contributors

1. Fork this repository and clone the forked repository to your local machine.
2. Create a new branch in your workspace:

   ```bash
   git checkout -b new_branch
   ```

3. Make revisions.
4. Save and push:

   ```bash
   git add *
   git commit -m "feat(scope): content"
   git push --set-upstream origin new_branch
   ```

5. Initiate a Pull Request.

## Prerequisites

To contribute to this project, you will need to have the following tools installed:

1. Git
2. Node.js
3. pnpm
4. Rust Cargo

## Setup Tauri

To setup Tauri, follow the instructions in the [Tauri Beta Documentation](https://v2.tauri.app/start/prerequisites/).

## Setup PNPM

1. Install pnpm globally:

   ```bash
   npm install -g pnpm
   ```

2. Install dependencies:

   ```bash
   pnpm install
   ```

## Launching the Project

### PC

```bash
pnpm tauri dev
```

### Android

```bash
pnpm tauri android dev
```

You should enable Rust cross-compilation environment for other platforms.
