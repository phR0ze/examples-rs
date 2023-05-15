# dioxus-bulma
Researching Dioxus with Bulma

## Resources

## Dioxus learning

### Global state
The `fermi` project provide a convenient way to access global state by object type.
However in order to be efficient we need to be careful in how we construct and use this
global state as Dioxus will be triggering the re-render of components based on their
use of state that has also changed. This means that it is problematic to use a centralized
state object for all state as any part of the system that uses the same state object will
automatically trigger component updates even when they should be triggered. As a result
the best practice for this is to use granular objects for state. Potentially a state
object per type or even more ganular depending. In this way we can avoid un-related
component re-renders.

## BACKLOG
* Icon support
* Better logging support

## COMPLETED
* Broke out Card components