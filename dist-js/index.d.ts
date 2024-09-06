import { Alias, Group, Identify, Page, Screen, Track } from "./bindings";
export * from "./bindings";
/**
 * Watch for URL changes and send analytics events.
 *
 * @returns A function to stop watching for URL changes.
 */
export declare const watchURLChanges: () => () => void;
/**
 * a page event
 * @param {Page} page
 */
export declare const sendPageEvent: (page: Page) => Promise<void>;
/**
 * a screen event
 * @param {Screen} screen
 */
export declare const sendScreenEvent: (screen: Screen) => Promise<void>;
/**
 * a track event
 * @param {Track} message
 */
export declare const sendTrackEvent: (message: Track) => Promise<void>;
/**
 * a identify event
 * @param {Identify} message
 */
export declare const sendIdentifyEvent: (message: Identify) => Promise<void>;
/**
 * a group event
 * @param {Group} message
 */
export declare const sendGroupEvent: (message: Group) => Promise<void>;
/**
 * a alias event
 * @param {Alias} message
 */
export declare const sendAliasEvent: (message: Alias) => Promise<void>;
