export enum WeightPlateSize {
    STD = "weight-plate-std",
    MEDIUM = "weight-plate-med",
    SMALL = "weight-plate-sml",
}

export enum WeightPlateColor {
    RED = "weight-plate-red",
    BLUE = "weight-plate-blue",
    YELLOW = "weight-plate-yellow",
    GREEN = "weight-plate-green",
    WHITE = "weight-plate-white",
    BLACK = "weight-plate-black",
}

export class WeightPlateSet {
    displayName!: string;
    unit!: string;
    barbellWeights!: Array<number>;
    clipWeight!: number;
    weights!: Array<number>;
    weightsStyle!: Map<number, [WeightPlateSize, WeightPlateColor]>;
    decimalPlaces: number = 0;

    fmtWeightWithUnit(weight: number) {
        return `${weight / 10 * this.decimalPlaces} ${this.unit}`
    }

    parseWeight(weight: number) {
        return weight * 10 * this.decimalPlaces;
    }
}

export const STD_OLYMPIC_KG: WeightPlateSet = Object.assign(new WeightPlateSet(), {
    displayName: "Standard Olympic KG Plates",
    unit: "kg",
    barbellWeights: [200, 150],
    clipWeight: 25,
    weights: [5, 10, 15, 20, 25, 50, 100, 150, 200, 250],
    weightsStyle: new Map<number, [WeightPlateSize, WeightPlateColor]>([
        [250, [WeightPlateSize.STD, WeightPlateColor.RED]],
        [200, [WeightPlateSize.STD, WeightPlateColor.BLUE]],
        [150, [WeightPlateSize.STD, WeightPlateColor.YELLOW]],
        [100, [WeightPlateSize.STD, WeightPlateColor.GREEN]],
        [50, [WeightPlateSize.MEDIUM, WeightPlateColor.WHITE]],
        [25, [WeightPlateSize.MEDIUM, WeightPlateColor.RED]],
        [20, [WeightPlateSize.SMALL, WeightPlateColor.BLUE]],
        [15, [WeightPlateSize.SMALL, WeightPlateColor.YELLOW]],
        [10, [WeightPlateSize.SMALL, WeightPlateColor.GREEN]],
        [5, [WeightPlateSize.SMALL, WeightPlateColor.WHITE]],
    ]),
    decimalPlaces: 1,
});

export const STD_OLYMPIC_LB: WeightPlateSet = Object.assign(new WeightPlateSet(), {
    displayName: "Olympic LB Plates",
    unit: "lb",
    barbellWeights: [450, 350],
    clipWeight: 50,
    weights: [5, 10, 25, 50, 100, 250, 350, 450, 550],
    weightsStyle: new Map<number, [WeightPlateSize, WeightPlateColor]>([
        [550, [WeightPlateSize.STD, WeightPlateColor.RED]],
        [450, [WeightPlateSize.STD, WeightPlateColor.BLUE]],
        [350, [WeightPlateSize.STD, WeightPlateColor.YELLOW]],
        [250, [WeightPlateSize.STD, WeightPlateColor.GREEN]],
        [100, [WeightPlateSize.MEDIUM, WeightPlateColor.WHITE]],
        [50, [WeightPlateSize.MEDIUM, WeightPlateColor.RED]],
        [25, [WeightPlateSize.SMALL, WeightPlateColor.BLUE]],
        [10, [WeightPlateSize.SMALL, WeightPlateColor.YELLOW]],
        [5, [WeightPlateSize.SMALL, WeightPlateColor.GREEN]],
    ]),
    decimalPlaces: 1,
});

export const GYM_PLATES_KG: WeightPlateSet = Object.assign(new WeightPlateSet(), {
    displayName: "Gym KG Plates",
    unit: "kg",
    barbellWeights: [200, 150],
    clipWeight: 25,
    weights: [10, 15, 20, 25, 50, 100, 150, 200],
    weightsStyle: new Map<number, [WeightPlateSize, WeightPlateColor]>([
        [200, [WeightPlateSize.STD, WeightPlateColor.BLUE]],
        [150, [WeightPlateSize.STD, WeightPlateColor.YELLOW]],
        [100, [WeightPlateSize.STD, WeightPlateColor.GREEN]],
        [50, [WeightPlateSize.STD, WeightPlateColor.WHITE]],
        [25, [WeightPlateSize.SMALL, WeightPlateColor.RED]],
        [20, [WeightPlateSize.SMALL, WeightPlateColor.BLUE]],
        [15, [WeightPlateSize.SMALL, WeightPlateColor.YELLOW]],
        [10, [WeightPlateSize.SMALL, WeightPlateColor.GREEN]],
    ]),
    decimalPlaces: 1,
});

export const GYM_PLATES_LB: WeightPlateSet = Object.assign(new WeightPlateSet(), {
    displayName: "Gym LB Plates",
    unit: "lb",
    barbellWeights: [450, 350],
    clipWeight: 50,
    weights: [25, 50, 100, 250, 350, 450],
    weightsStyle: new Map<number, [WeightPlateSize, WeightPlateColor]>([
        [450, [WeightPlateSize.STD, WeightPlateColor.BLUE]],
        [350, [WeightPlateSize.STD, WeightPlateColor.YELLOW]],
        [250, [WeightPlateSize.STD, WeightPlateColor.GREEN]],
        [100, [WeightPlateSize.STD, WeightPlateColor.BLACK]],
        [50, [WeightPlateSize.MEDIUM, WeightPlateColor.BLACK]],
        [25, [WeightPlateSize.SMALL, WeightPlateColor.BLACK]],
    ]),
    decimalPlaces: 1,
});
