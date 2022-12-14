export function capitalize(str: string) {
    return str.charAt(0).toUpperCase() + str.slice(1);
}

// e.g. "name" -> "setName"
export function propertyNameToSetterName(property: string): string {
    return `set${capitalize(property)}`
}

export function singularToPlural(singular: string): string {
    if(singular.endsWith("y")) {
        return singular.slice(0, -1) + "ies"
    } else {
        return singular + "s"
    }
}

export function pluralToSingular(plural: string): string {
    if(plural.endsWith("ies")) {
        return plural.slice(0, -3) + "y"
    } else if(plural.endsWith("s")) {
        return plural.slice(0, -1)
    } else {
        return plural
    }
}

// e.g. "comments" -> "addComment"
export function collectionToAdderName(collection: string): string {
    return `add${capitalize(pluralToSingular(collection))}`
}

export function stringifyObjectLiteral(obj) {
    if(Array.isArray(obj)) {
        //@ts-ignore
        return `[${obj.map(stringifyObjectLiteral).join(", ")}]`
    }
    
    const keys = Object.keys(obj);
    const stringifiedPairs = [];
  
    for (const key of keys) {
      const valueString = JSON.stringify(obj[key]);
      const keyValuePairString = `${key}: ${valueString}`;
      stringifiedPairs.push(keyValuePairString);
    }

    return `{${stringifiedPairs.join(', ')}}`;
  }