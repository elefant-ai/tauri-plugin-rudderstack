/** user-defined commands **/
export declare const commands: {
    /**
     * Send an analytics event to the RudderStack data plane.
     */
    sendAnalyticsAlias(event: Alias): Promise<void>;
    /**
     * Send an analytics event to the RudderStack data plane.
     */
    sendAnalyticsGroup(event: Group): Promise<void>;
    /**
     * Send an [Identify] event to the RudderStack data plane.
     */
    sendAnalyticsIdentify(event: Identify): Promise<void>;
    /**
     * Send a [Page] event to the RudderStack data plane.
     */
    sendAnalyticsPage(event: Page): Promise<void>;
    /**
     * Send a [Screen] event to the RudderStack data plane.
     */
    sendAnalyticsScreen(event: Screen): Promise<void>;
    /**
     * Send a [Track] event to the RudderStack data plane.
     */
    sendAnalyticsTrack(event: Track): Promise<void>;
};
/** user-defined events **/
/** user-defined constants **/
/** user-defined types **/
/**
 * An alias event.
 *
 * The `alias` call lets you merge different identities of a known user. \
 *
 * Alis is an advanced method that lets you change the tracked user's ID explicitly. This method is useful when managing identities for some of the downstream destinations.
 *
 */
export type Alias = {
    /**
     * The user id associated with this message.
     */
    userId: string;
    /**
     * The user's previous ID.
     */
    previousId: string;
    /**
     * The traits to assign to the alias.
     */
    traits?: JsonValue | null;
    /**
     * The timestamp associated with this message.
     */
    originalTimestamp?: string | null;
    /**
     * Context associated with this message.
     */
    context?: JsonValue | null;
    /**
     * Integrations to route this message to.
     */
    integrations?: JsonValue | null;
};
/**
 * A group event.
 * The `group` call lets you associate an identified user to a group - either a company, project or a team and record any custom traits or properties associated with that group. \
 * An identified user can be in more than one group.
 */
export type Group = {
    /**
     * The group the user is being associated with.
     */
    groupId: string;
    /**
     * The traits to assign to the group.
     */
    traits?: JsonValue | null;
    /**
     * The timestamp associated with this message.
     */
    originalTimestamp?: string | null;
    /**
     * Context associated with this message.
     */
    context?: JsonValue | null;
    /**
     * Integrations to route this message to.
     */
    integrations?: JsonValue | null;
};
/**
 * An identify event.
 * The identify call lets you identify a visiting user and associate them to their actions. It also lets you record the traits about them like their name, email address, etc.
 */
export type Identify = {
    /**
     * The traits to assign to the user.
     */
    traits?: JsonValue | null;
    /**
     * The timestamp associated with this message.
     */
    originalTimestamp?: string | null;
    /**
     * Context associated with this message.
     */
    context?: JsonValue | null;
    /**
     * Integrations to route this message to.
     */
    integrations?: JsonValue | null;
};
export type JsonValue = null | boolean | number | string | JsonValue[] | {
    [key in string]: JsonValue;
};
/**
 * A page event.
 *
 * The page call allows you to record the page views on your website along with the other relevant information about the viewed page.
 * RudderStack recommends calling page at least once every page load.
 */
export type Page = {
    /**
     * The name of the page being tracked.
     */
    name: string;
    /**
     * The properties associated with the event.
     */
    properties?: JsonValue | null;
    /**
     * The timestamp associated with this message.
     */
    originalTimestamp?: string | null;
    /**
     * Context associated with this message.
     */
    context?: JsonValue | null;
    /**
     * Integrations to route this message to.
     */
    integrations?: JsonValue | null;
};
/**
 * A screen event.
 *
 * The screen call is the mobile equivalent of the page call.
 *
 * The screen method lets you record whenever the user views their mobile screen, along with any additional relevant information about the screen.
 * The screen call is the mobile equivalent of the page call.
 */
export type Screen = {
    /**
     * The name of the screen being tracked.
     */
    name: string;
    /**
     * The properties associated with the event.
     */
    properties?: JsonValue | null;
    /**
     * The timestamp associated with this message.
     */
    originalTimestamp?: string | null;
    /**
     * Context associated with this message.
     */
    context?: JsonValue | null;
    /**
     * Integrations to route this message to.
     */
    integrations?: JsonValue | null;
};
/**
 * A track event.
 * The track call lets you record the user actions along with their associated properties. Each user action is called an event.
 */
export type Track = {
    /**
     * The name of the event being tracked.
     */
    event: string;
    /**
     * The properties associated with the event.
     */
    properties?: JsonValue | null;
    /**
     * The timestamp associated with this message.
     */
    originalTimestamp?: string | null;
    /**
     * Context associated with this message.
     */
    context?: JsonValue | null;
    /**
     * Integrations to route this message to.
     */
    integrations?: JsonValue | null;
};
export type Result<T, E> = {
    status: "ok";
    data: T;
} | {
    status: "error";
    error: E;
};
