#include <iostream>
#include <vector>
#include <string>
#include <math.h>
using namespace std;

struct Match {
	int bracket;
	int round;
	string id;
	string upper;
	string lower;
};

string to_string(Match m) {
	return m.id + " - " + m.upper + " VS " + m.lower;
}

int main() {
	long int numParticipants = 32768;
	vector<string> participants;
	string participant;
	/*
	cout << "~~~ BRACKET GENERATOR ~~~" << endl;

	cout << "How many participants (n >= 2)? ";
	cin >> numParticipants;
	cin.ignore();
	cout << endl;
	
	cout << "Enter the participants:" << endl;*/
	for (unsigned int i = 0; i < numParticipants; i++) {
		/*cout << "Seed " << i + 1 << ": ";
		getline(cin, participant);
		cout << endl;*/

		participants.push_back(to_string(i + 1));
	}

	int numMatches = numParticipants - 1;
	vector<Match> matches = vector<Match>(numMatches);
	int numRounds = log2(numParticipants);

	if (numParticipants != pow(2, numRounds)) {
		numRounds++;
	}

	matches.at(numMatches - 1).bracket = 1;
	matches.at(numMatches - 1).round = numRounds;
	matches.at(numMatches - 1).upper = participants.at(0);
	matches.at(numMatches - 1).lower = participants.at(1);
	
	int opponentSeed = pow(2, numRounds) - numParticipants + 1;
	int round = 1;
	int match = 1;
	for (int i = numParticipants; i > 1; i--) {
		double testNum = log2(i);
		int testNum2 = testNum;
		if (testNum == testNum2) {
			opponentSeed = 1;
			round++;
			match = 1;
		}

		matches.at(numParticipants - i).upper = participants.at(opponentSeed - 1);
		matches.at(numParticipants - i).lower = participants.at(i - 1);
		cout << participants.at(opponentSeed - 1) << endl;
		matches.at(numParticipants - i).round = round;
		matches.at(numParticipants - i).bracket = 1;
		matches.at(numParticipants - i).id = "B" + to_string(1) + "R" + to_string(round) + "M" + to_string(match);

		participants.at(opponentSeed - 1) = "Winner of " + matches.at(numParticipants - i).id;

		cout << to_string(matches.at(numParticipants - i)) << endl;

		opponentSeed++;
		match++;
	}

	return 0;
}