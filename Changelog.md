# Changelog

Like the captain's log from Star Trek except I'm not English or Bald, nor in space, nor a captain.

### May 18 2025

#### 9:33 AM

- Finally added a keybind for Copilot Doc generation, documented all client functions for `cargo docs`
- Hopefully they're good, I ain't reading all that
  - should do the same for tests, I guess
- Added an ApiComponent Struct for adding Client-side API routes.
- Modified some write logic, extracted some reusable logic, etc etc the usual
- Ready to dryfire the general `install auth` command

#### 11:50 AM

- Wrestled with cargo, added cliclack UI updates and multithreading to `new` and `feat` commands
- Current task: finishing Auth module imports

### May 17 2025

#### 1:08 PM

- Chased around some different API schemas yesterday, landed somewhere good, I think.
- Wrote `sona` and `vona` axios instances and various utils for adding handshake and user tokens to requests via axios 'middleware' - just sanity stuff for denying malignant users, bots, scrapers, etc.
- Expanded the AuthState component to hold signals for all forms, very nice devex there that I hope to extend and model on other modules as needed.
- Wrote various front end components, expanded others. The TextInput and PasswordInput in particular were upgraded to accept Zod validation.
- Current working directory of new components includes:
  - Card
  - Drawer (incomplete)
  - ErrorMessage, which takes a "when" prop - used when the error message signal is not null
  - Heading, which takes an as prop to return different tags with different, configurable default styles
  - P, generic paragraph wrapper with default styling
  - Page, generic main wrapper with default (immutable!) styling
  - TopNav, which needs some extension to be more configurable
  - FooterNav, which also needs config support
  - PageInner, which composes the navs and the child elements
  - Stack, which takes a `direction` prop for fast, prestyled column or row flexboxes

#### 4:32 PM

- added most of those components
- The Navs are still not implemented, I have no good ideas and I'm not asking AI to architect for me.
  - Need more idle time for thinking?

### May 15 2025

#### 7:47 AM

- Moved main registry to components/atoms, will copy pattern across future molecules and features
- Added state enum for storing client-side state controllers
- Added decorators enum for storing purely decorative elements, currently, just horizontal/vertical separators, optionally with or without a label
- Cleaned up UI side, components are organized loosely as either an ATOM or a FEATURE component
- ATOMs are extracted, repeating components from FEATUREs
- FEATURE components compose ATOMs plus one-off layout + data to do WORK
- I love WORK
- Still not totally sure what to do with things like Utils or State
- They go in their own top level folder for now, but State might be a thing that gets moved to FEATUREs later

#### 9:08 AM

- Removed State from ATOMs, moved the only existing state, `AuthState` to new library/client/components/features/auth.rs
- Basic impl of auth in auth.rs, todo: Password reset form, password challenge form
- Todo: Think about adding SSO
- This thing builds, but a lot of code is unused
- Going to continue writing the Auth feature client side for now, copying over any improvements. Specifically thinking about structured fetch requests, click handlers, etc.

#### 3:07 PM

- Looked at TRPC, only really useful for monorepo's like solid start.
- The plan for Devano is to use SolidStart/NextJS for things like Backend For Frontend - dynamic routes in certain modules pull full json pageloads, dynamic landing pages from a json spec, etc.
- Decided Devano should support arbitrary backends (future feature to export features to Python/Rust/C backends, staying Typescript/Express + FileRouter for now)

### May 14 2025

#### 12:41 PM

- Added a `folder_path` field to the UiComponent struct because, while I was cooking up components, I noticed that having everything be a sibling was created a big fat f-mess
- Decided to remove the `COMPONENT` from the component definition variable names - it's kinda redundant.
- Now that I say this outloud, writing out "literal" is probably also redundant.
- In fact, I am going to redeclare the component system to be an enum and declare the literals inside some kind of enum impl so I don't have to deal with this anymore.

#### 3:05 PM

- I did not declare the literals in-line, but we do have an enum impl for the atoms
