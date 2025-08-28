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
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import org.openqa.selenium.WebElement
import com.kms.katalon.core.webui.common.WebUiCommonHelper


// Nhập keyword từ Excel
String searchKeyword = keyword.toLowerCase()

WebUI.openBrowser('')

WebUI.navigateToUrl('https://thegioiskinfood.com/')

WebUI.click(findTestObject('Object Repository/Tìm kiếm/Page_Th Gii SkinFood - M phm chnh hng  TH G_5b2e13/div_T kha ph binromandsonkem chng nngmt nke_7e2d68'))

WebUI.click(findTestObject('Object Repository/Tìm kiếm/Page_Th Gii SkinFood - M phm chnh hng  TH G_5b2e13/input_Dch_q'))

WebUI.setText(findTestObject('Object Repository/Tìm kiếm/Page_Th Gii SkinFood - M phm chnh hng  TH G_5b2e13/input_Blind Box - Lucky Emma_q'), 
    keyword)

WebUI.click(findTestObject('Object Repository/Tìm kiếm/Page_Th Gii SkinFood - M phm chnh hng  TH G_5b2e13/button_Blind Box - Lucky Emma_btn'))

// Đợi kết quả tải
WebUI.delay(3)

// Sau khi nhấn tìm kiếm, mới kiểm tra keyword rỗng và expected có giá trị → ép fail
if (searchKeyword.trim() == '' && expectedresult.trim() != '' && expectedresult.toLowerCase() != 'no result') {
	WebUI.comment("Keyword rỗng nhưng expected result lại có nội dung: '" + expectedresult + "'")
	WebUI.verifyMatch('', expectedresult, false)
}

// Kiểm tra có kết quả hay không
boolean isNoResult = WebUI.verifyElementPresent(
    findTestObject('Object Repository/Tìm kiếm/Page_Kt qu tm kim 1 - TH GII SKINFOOD/no_resultsearch'),
    3,
    FailureHandling.OPTIONAL
)

if (isNoResult) {
    WebUI.comment("Không có kết quả tìm kiếm cho từ khóa: " + keyword)

    // Lấy nội dung hiển thị thực tế từ trang web
    String actualMessage = WebUI.getText(findTestObject('Object Repository/Tìm kiếm/Page_Kt qu tm kim 1 - TH GII SKINFOOD/no_resultsearch')).trim()
    WebUI.comment("Không có kết quả tìm kiếm cho từ khóa: " + keyword)
    WebUI.comment("Thông báo hiển thị: " + actualMessage)
	// So sánh thông báo tìm kiếm với expected result từ Excel
    WebUI.verifyMatch(actualMessage.toLowerCase(), expectedresult.toLowerCase().trim(), false)
   
} else {
	// Không có phần tử "no result" → Kiểm tra danh sách kết quả
		List<WebElement> productTitles = WebUiCommonHelper.findWebElements(
		findTestObject('Object Repository/Tìm kiếm/Page_Kt qu tm kim 1 - TH GII SKINFOOD/product_title_list'),
		10
	)
	if (productTitles.size() == 0) {
		// Không có sản phẩm nào được hiển thị
		WebUI.comment("Không có kết quả nào và cũng không có thông báo từ hệ thống.")

		if (expectedresult != null && expectedresult.trim() != '' && expectedresult.toLowerCase().trim() != 'no result') {
			WebUI.comment("Expected: '" + expectedresult + "' nhưng KHÔNG có thông báo nào hiển thị.")
			// Ép fail
			WebUI.verifyMatch("", expectedresult, false)
		} else {
			WebUI.comment("Không có kết quả & không có thông báo, và expected rỗng hoặc 'no result' → PASS")
		}

	} else {
    
    boolean allMatch = true
        for (WebElement item : productTitles) {
            String productName = item.getText().toLowerCase()
            if (productName.contains(searchKeyword)) {
                WebUI.comment("Có chứa keyword: " + productName)
            } else {
                WebUI.comment("KHÔNG chứa keyword: " + productName)
                allMatch = false
            }
        }

        WebUI.verifyEqual(allMatch, true)
    }
}

WebUI.closeBrowser()

