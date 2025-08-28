import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

// Biến truyền từ file Excel (Data-driven)
WebUI.openBrowser('')

WebUI.navigateToUrl('https://thegioiskinfood.com/')

WebUI.click(findTestObject('Object Repository/GioHang/Page_Th Gii SkinFood - M phm chnh hng  TH G_5b2e13/a_Ty T Bo Cht Rosette Chit Xut T Thin Nhin _45efec'))

WebUI.click(findTestObject('Object Repository/GioHang/Page_Ty T Bo Cht Chit Xut T Thin Nhin Nht B_89bf39/span_DNG SNG DA (BN MI)'))

WebUI.click(findTestObject('Object Repository/GioHang/Page_Ty T Bo Cht Chit Xut T Thin Nhin Nht B_89bf39/button_'))

WebUI.click(findTestObject('Object Repository/GioHang/Page_Ty T Bo Cht Chit Xut T Thin Nhin Nht B_89bf39/button_'))

WebUI.click(findTestObject('Object Repository/GioHang/Page_Ty T Bo Cht Chit Xut T Thin Nhin Nht B_89bf39/button_-'))

WebUI.click(findTestObject('Object Repository/GioHang/Page_Ty T Bo Cht Chit Xut T Thin Nhin Nht B_89bf39/button_Thm vo gi'))

WebUI.verifyElementText(findTestObject('Object Repository/GioHang/Page_Ty T Bo Cht Chit Xut T Thin Nhin Nht B_89bf39/span_Sn phm  c thm vo gi hng'), 
    'Sản phẩm đã được thêm vào giỏ hàng')

WebUI.closeBrowser()

