import { Alias, commands, Group, Identify, Page, Screen, Track } from "./bindings";
export * from "./bindings";

interface PageProperties {
    title: string;
    url: string;
    path: string;
}

const getPageProperties = (): PageProperties => {
    return {
        title: document.title,
        url: window.location.href,
        path: window.location.pathname,
    }
};


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
        const props = getPageProperties();
        sendPageEvent({
            name: props.path,
            properties: {
                title: props.title,
                url: props.url,
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

const addPageProperties = (message: Track) => {
    const pageProperties = getPageProperties();

    // Ensure message.properties is initialized as an object
    if (!message.properties || typeof message.properties !== 'object') {
        message.properties = {};
    }

    // Cast properties to an object for type safety
    const properties = message.properties as { [key: string]: any };

    // Merge the page properties into the properties field
    properties.page = {
        ...(properties.page || {}),
        ...pageProperties,
    };

    // Assign the updated properties back to the message
    message.properties = properties;
    return message;
}


/**
 * a track event
 * @param {Track} message
 */
export const sendTrackEvent = async (message: Track) => {
    const msg = addPageProperties(message);
    await commands.sendAnalyticsTrack(msg);
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
