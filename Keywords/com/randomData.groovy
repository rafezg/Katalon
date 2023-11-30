package com

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
//import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import org.openqa.selenium.Keys as Keys

import internal.GlobalVariable

public class RandomName {

	@Keyword
	def CreateRandomName(def name) {
		String random_name = name + new Random().nextInt(999 - 1 + 1)
		return (random_name)
	}
}

public class ClearField {

	@Keyword
	def FieldName(def arg) {
		WebUI.sendKeys(findTestObject(arg), Keys.chord(Keys.CONTROL, 'a')) "Only works on date fields"
		WebUI.sendKeys(findTestObject(arg), Keys.chord(Keys.BACK_SPACE)) "Only works on date fields"
	}
}


// Used for creating a Random numbers using current date and time at runtime

public class RandomNumber {

	@Keyword
	def CreateRandomNumber(def state) {
		Date todaysDate = new Date();
		def formattedDate = todaysDate.format("yyyyMMddHHmmss");
		return (formattedDate)
	}
}

// Used for creating a Random last name at runtime

public class RandomLastName {

	@Keyword
	def CreateRandomLastName(def state) {
		Random rand = new Random()
		List<String> givenList = Arrays.asList("Jones", "Smith", "Fraser", "Duck", "Micky", "Vader", "Skywalker", "Abood", "Walsh", "Yoda", "Johnson")

		int numberOfElements = 11

		for (int i = 0;  i < numberOfElements; i++) {
			int randomIndex = rand.nextInt(givenList.size())
			String randomElement = givenList.get(randomIndex)
			return (randomElement)
		}
	}
}

// Used for creating a Random Australian state at runtime

public class RandomState {

	@Keyword
	def CreateRandomState(def state) {
		Random rand = new Random()
		List<String> givenList = Arrays.asList("QLD", "NSW", "VIC", "TAS", "WA", "NT", "ACT")

		int numberOfElements = 7

		for (int i = 0;  i < numberOfElements; i++) {
			int randomIndex = rand.nextInt(givenList.size())
			String randomElement = givenList.get(randomIndex)
			return (randomElement)
		}
	}
}

// Used for creating a Random Australian city at runtime

public class RandomCity {

	@Keyword
	def CreateRandomCity(def city) {
		Random rand = new Random()
		List<String> givenList = Arrays.asList("Brisbane", "Sydney", "Gladstone", "Townsville", "Perth", "Darwin", "Gympie")

		int numberOfElements = 7

		for (int i = 0;  i < numberOfElements; i++) {
			int randomIndex = rand.nextInt(givenList.size())
			String randomElement = givenList.get(randomIndex)
			return (randomElement)
		}
	}
}

// Used for creating a Random Australian street at runtime

public class RandomStreet {

	@Keyword
	def CreateRandomStreet(def street) {
		Random rand = new Random()
		List<String> givenList = Arrays.asList("Williams", "East", "South", "West", "North", "Claremont", "Clark")

		int numberOfElements = 7

		for (int i = 0;  i < numberOfElements; i++) {
			int randomIndex = rand.nextInt(givenList.size())
			String randomElement = givenList.get(randomIndex)
			return (randomElement)
		}
	}
}

// Used for creating a Random Australian street at runtime

public class RandomCountry {

	@Keyword
	def CreateRandomCountry(def street) {
		Random rand = new Random()
		List<String> givenList = Arrays.asList("Australia", "New Zealand", "USA", "England", "Canada", "Mexico", "PNG")

		int numberOfElements = 7

		for (int i = 0;  i < numberOfElements; i++) {
			int randomIndex = rand.nextInt(givenList.size())
			String randomElement = givenList.get(randomIndex)
			return (randomElement)
		}
	}
}

// Used for creating a Random Australian street at runtime

public class RandomPhrase {

	@Keyword
	def CreateRandomPhrase(def street) {
		Random rand = new Random()
		List<String> givenList = Arrays.asList("I am out of the office", "I will be on PTO", "Gone fishing", "Recreation Leave", "Sick Leave", "Gone Camping", "Moved to Mars")

		int numberOfElements = 7

		for (int i = 0;  i < numberOfElements; i++) {
			int randomIndex = rand.nextInt(givenList.size())
			String randomElement = givenList.get(randomIndex)
			return (randomElement)
		}
	}
}