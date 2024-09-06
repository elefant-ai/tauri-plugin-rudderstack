import { Alias, commands, Group, Identify, Page, Screen, Track } from "./bindings";
export * from "./bindings";


/**
 * Watch for URL changes and send analytics events.
 *
 * @returns A function to stop watching for URL changes.
 */
export const watchURLChanges = () => {
    const rs = history.pushState;
    history.pushState = function () {
        // @ts-expect-error
        rs.apply(history, arguments); // preserve normal functionality
        sendPageEvent({
            name: window.location.pathname,
            properties: {
                title: document.title,
                url: window.location.href
            },
        });
    };

    return () => {
        history.pushState = rs;
    }
};

/**
 * a page event
 * @param {Page} page
 */
export const sendPageEvent = async (page: Page) => {
    await commands.sendAnalyticsPage(page);
}

/**
 * a screen event
 * @param {Screen} screen
 */
export const sendScreenEvent = async (screen: Screen) => {
    await commands.sendAnalyticsScreen(screen);
}

/**
 * a track event
 * @param {Track} message
 */
export const sendTrackEvent = async (message: Track) => {
    await commands.sendAnalyticsTrack(message);
}

/**
 * a identify event
 * @param {Identify} message
 */
export const sendIdentifyEvent = async (message: Identify) => {
    await commands.sendAnalyticsIdentify(message);
}

/**
 * a group event
 * @param {Group} message
 */
export const sendGroupEvent = async (message: Group) => {
    await commands.sendAnalyticsGroup(message);
}

/**
 * a alias event
 * @param {Alias} message
 */
export const sendAliasEvent = async (message: Alias) => {
    await commands.sendAnalyticsAlias(message);
}
