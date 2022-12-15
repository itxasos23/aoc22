from rich import print
from dataclasses import dataclass

@dataclass
class Beacon:
    x: int
    y: int
    
    def __hash__(self):
        return self.x * 10000000 + self.y

    def __eq__(self, other):
        return self.__hash__() == other.__hash__()

@dataclass
class Sensor:
    x: int
    y: int
    beacon: Beacon



class AOC2215:
    def __init__(self, filename):
        self.filename = filename
        with open(filename) as fd:
            data = fd.read().strip().splitlines()

        self.sensors = []
        for line in data:
            sensor_x = int(line.split(': ')[0].split('Sensor at x=')[1].split(',')[0])
            sensor_y = int(line.split(': ')[0].split('y=')[1])

            beacon_x = int(line.split('beacon is at x=')[1].split(', ')[0])
            beacon_y = int(line.split('y=')[2].strip())

            self.sensors.append(
                Sensor(
                    x=sensor_x,
                    y=sensor_y,
                    beacon=Beacon(
                        x=beacon_x,
                        y=beacon_y
                    )
                )
            )

        all_x = [s.x for s in self.sensors] + [s.beacon.x for s in self.sensors]
        all_y = [s.y for s in self.sensors] + [s.beacon.y for s in self.sensors]

        self.min_x = min(all_x)
        self.max_x = max(all_x)
        self.min_y = min(all_y)
        self.max_y = max(all_y)


    def print_setup(self):
        min_x = self.min_x - 20
        max_x = self.max_x + 20
        min_y = self.min_y - 20
        max_y = self.max_y + 20
        
        print_strs = [] 
        for y in range(min_y, max_y+1):
            print_row = ""
            for x in range(min_x, max_x+1):
                if (x, y) in [(s.x, s.y) for s in self.sensors]:
                    print_row += 'S'
                elif (x, y) in [(s.beacon.x, s.beacon.y) for s in self.sensors]:
                    print_row += 'B'
                elif self.is_point_covered(x, y):
                    print_row += '#'
                else:
                    print_row += '.'
            print_strs.append(print_row)

        print('\n'.join(print_strs))


    def is_point_covered(self, x, y):
        for sensor in self.sensors:
            if (x, y) == (sensor.beacon.x, sensor.beacon.y):
                return False  # it's not "covered" as in, there's a beacon here.

        for sensor in self.sensors:
            md_to_sensor = abs(x - sensor.x) + abs(y - sensor.y)
            md_sensor_to_beacon = abs(sensor.x - sensor.beacon.x) + abs(sensor.y - sensor.beacon.y)
            if md_to_sensor <= md_sensor_to_beacon:
                return True

        return False

    def find_covered_blocks_in_row(self, y):
        covered_sets = []
        for sensor in self.sensors:
            sensor_md_radius = abs(sensor.x - sensor.beacon.x) + abs(sensor.y - sensor.beacon.y)
            coverage_width = sensor_md_radius - abs(sensor.y - y)
            if coverage_width <= 0:
                continue


            coverage_set = [sensor.x - coverage_width, sensor.x + coverage_width]
            coverage_set.sort()
            covered_sets.append(coverage_set)

        covered_sets.sort(key=lambda x: x[0])
        # print(covered_sets)

        new_covered_sets = [covered_sets[0]]
        for c_set in covered_sets[1:]:
            if new_covered_sets[-1][1] < c_set[0]-1:
                # print(f"We have a gap! - {new_covered_sets[-1], c_set}")
                new_covered_sets.append(c_set)
            else:
                new_covered_sets[-1][1] = max(new_covered_sets[-1][1], c_set[1])

        #print(new_covered_sets)
        return new_covered_sets

    def part_1(self):
        if "ex" in self.filename:
            check_row = 10
        else:
            check_row = 2000000

        covered_sets = self.find_covered_blocks_in_row(check_row)
        beacons_covered = 0
        for beacon in set(x.beacon for x in self.sensors):
            for _set in covered_sets:
                if beacon.y == check_row and _set[0] <= beacon.y <= _set[1]:
                    # print(f"Beacon {beacon} covered in {(_set), check_row}")
                    beacons_covered += 1
                    break

        # print(beacons_covered)

        return sum(c[1] + 1 - c[0] for c in covered_sets) - beacons_covered

    def part_2(self):
        if "ex" in self.filename:
            max_y = 20
        else:
            max_y = 4000000

        for y in range(0, max_y + 1):
            covered_sets = self.find_covered_blocks_in_row(y)

            useful_sets = []
            for _set in covered_sets:
                useful_sets.append((max(_set[0], 0), min(_set[1], max_y)))

            sum_value = sum(s[1] - s[0] for s in useful_sets)
            if sum_value < max_y:
                #print(f'beacon found: {useful_sets, y}')
                x = useful_sets[0][1] + 1
                return x * 4000000 + y



if __name__ == '__main__':
    aoc = AOC2215("input_ex.txt")
    print(aoc.part_1())
    aoc = AOC2215("input.txt")
    print(aoc.part_1())
    aoc = AOC2215("input_ex.txt")
    print(aoc.part_2())
    aoc = AOC2215("input.txt")
    print(aoc.part_2())
