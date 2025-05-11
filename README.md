# Devano/CLI

![Vibe Coded](https://img.shields.io/badge/vibe-coded-622118)
![Hyper Ware](https://img.shields.io/badge/hyper-ware-71DEF4)

A command line interface for rapid scaffolding. Vibe coded and pre-alpha.

## Why?

I've built a couple apps now, maybe 50 or 55 or so, and I am personally sick and tired of repeating myself.
Devano/CLI is like a combination design system (with big opinions on things like color, animation) and component/module library (with other big opinions on things like API design, Auth, payments, etc.)

## Project goals

This is a ridiculous statement due to atemporal recursion, but:
Devano/CLI will be complete when it reduces MVP app development time by 80%.

As soon as this is achieved, of course, it can't happen again. So 80% faster than doing a by-hand, full stack build circa 2024, which is 3-8 weeks ish. The target time is 2-6... it all depends on your core differentiating feature, which you'll have to build yourself.

But, all that other annoying stuff - the stupid cookie banner and friggen auth and stupid stripe integrations etc - that'll be like a 5 minute install when this is done.

## Current Features

This is _super_ pre-alpha, just want a document showing the development timeline, more or less.

Currently, the CLI does the following:

- Pretty `cliclack` integration for navigating
- `devano new` scaffolds a basic full-stack app
- `devano ui` gives you a submenu for doing UI stuff.
  - currently, only the css/color/simple flow is 'done'
  - that flow allows you to supply a single hex code and does a ton of processing to give a 32-tone palette
  - the palette is spit out as css into your project giving you an instant light/dark theme for use with future devano/cli components
  - you can also choose a rust export to add your own named themes to the selection menu

## Prerequisites

This builds a Full Stack Typescript app.
You gotta have `cargo` and `rust` installed, and also `pnpm`, which has it's own requirement stack that you'll have to dig through.

## Installing

Pre alpha baby!!!

Download this, have rust installed, run `cargo install --path . --force`.
